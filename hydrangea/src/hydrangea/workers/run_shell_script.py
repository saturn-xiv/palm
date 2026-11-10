import logging
import signal
import subprocess
from datetime import datetime, timedelta, UTC
from pathlib import Path


from ..etc import rabbitmq as open_rabbitmq, is_stopped
from ..protocols import email_pb2, hydrangea_pb2
from . import time_it, to_timestamp, to_duration, render, protobuf_message_properties

logger = logging.getLogger(__name__)


def launch(config, queue):
    if is_stopped():
        return

    working_dir = Path(config['working-dir'])
    email_queue = config['marigold']['email-queue']

    channel = open_rabbitmq(config)
    channel.queue_declare(queue=queue, durable=True,
                          arguments={})
    channel.basic_qos(prefetch_count=1)

    def callback(channel, method, properties, body):
        logger.info("receive message %s %s",
                    properties.message_id, properties.content_type)
        channel.basic_ack(delivery_tag=method.delivery_tag)

        launched_at = datetime.now(UTC)

        task = hydrangea_pb2.Task()
        task.ParseFromString(body)
        result, elapsed = execute(working_dir, task)
        logger.debug("%d \n%s\n%s", result.returncode,
                     result.stdout, result.stderr)
        req = hydrangea_pb2.StashJobRequest(
            task=task,
            status_code=result.returncode, stdout=result.stdout, stderr=result.stderr,
            launched_at=to_timestamp(launched_at),
            elapsed=to_duration(elapsed),
        )
        send_mail(channel, email_queue, config['cc'],
                  config['bcc'], req, timedelta(seconds=elapsed))
        # TODO report by grpc

    def handle_shutdown(signum, frame):
        logger.warning(
            "received signal %d, waiting for shutdown...", signum)
        channel.stop_consuming()

    if is_stopped():
        channel.stop_consuming()
    channel.basic_consume(
        queue=queue, on_message_callback=callback, auto_ack=False)

    signal.signal(signal.SIGTERM, handle_shutdown)
    signal.signal(signal.SIGINT, handle_shutdown)

    logging.info("waiting for messages. To exit press CTRL+C")
    channel.start_consuming()
    logger.info('exited')


@time_it
def execute(working_dir, task: hydrangea_pb2.Task):
    working_dir = working_dir / task.sn
    logger.debug("create working folder %s", working_dir)
    working_dir.mkdir(parents=True, exist_ok=False)

    shell = working_dir / "run.sh"
    logger.debug("generate shell file %s", shell)
    with open(shell, "w", encoding="utf-8") as f:
        f.write(task.script)
        f.flush()
    logger.debug("execute %s", shell)
    return subprocess.run(["/bin/sh", shell]+list(task.args), cwd=working_dir, capture_output=True, text=True)


def send_mail(channel, queue, cc, bcc, req, elapsed):
    args = {"req": req, "elapsed": elapsed}
    subject = render("shell-job-report/subject.mustache", args)
    body = render("shell-job-report/body.mustache", args)
    # logger.debug("%s\n%s", subject, body)

    task = email_pb2.Task(subject=subject.strip(),
                          body=email_pb2.Task.Body(html=True, content=body))
    for it in cc:
        task.cc.append(email_pb2.Task.Address(email=it))
    for it in bcc:
        task.bcc.append(email_pb2.Task.Address(email=it))

    properties = protobuf_message_properties()
    logger.debug("publish message to %s://%s/%s", queue,
                 properties.content_type, properties.message_id)
    channel.basic_publish(exchange='', routing_key=queue,
                          body=task.SerializeToString(), properties=properties)
