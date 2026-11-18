import logging
import signal
import subprocess
from datetime import datetime, timedelta, UTC
from pathlib import Path
import functools

import psycopg
import isodate

from ..etc import rabbitmq as open_rabbitmq, is_stopped, postgresql as open_postgresql
from ..protocols import email_pb2, hydrangea_pb2
from . import time_it, to_timestamp, to_duration, render, protobuf_message_properties

logger = logging.getLogger(__name__)


def launch(config, executor):
    if is_stopped():
        return

    working_dir = Path(config['working-dir'])
    queue = config['marigold']['hydrangea']['shell-run-queue']
    db_url = open_postgresql(config['marigold']['postgresql'])

    connection = open_rabbitmq(config['rabbitmq'])
    channel = connection.channel()
    channel.queue_declare(queue=queue, durable=True,
                          arguments={})
    channel.basic_qos(prefetch_count=1)

    on_message_callback = functools.partial(_on_message, args=(
        connection, executor, working_dir, config['cc'], config['bcc'], db_url))

    def handle_shutdown(signum, frame):
        logger.warning(
            "received signal %d, waiting for shutdown...", signum)
        channel.stop_consuming()

    if is_stopped():
        channel.stop_consuming()
    channel.basic_consume(
        queue=queue, on_message_callback=on_message_callback, auto_ack=False)

    signal.signal(signal.SIGTERM, handle_shutdown)
    signal.signal(signal.SIGINT, handle_shutdown)

    logging.info("waiting for messages. To exit press CTRL+C")
    channel.start_consuming()
    logger.info('exited')


@time_it
def _execute(working_dir, task: hydrangea_pb2.Task):
    working_dir = working_dir / task.sn
    logger.debug("create working folder %s", working_dir)
    working_dir.mkdir(parents=True, exist_ok=False)

    shell = working_dir / "run.sh"
    logger.debug("generate shell file %s", shell)
    with open(shell, "w", encoding="utf-8") as f:
        f.write(task.script)
        f.flush()
    logger.info("execute %s", shell)

    return subprocess.run(["/bin/bash", shell]+list(task.args), cwd=working_dir, capture_output=True, text=True)


def _build_email_task(cc, bcc, args):
    subject = render("shell-job-report/subject.mustache", args)
    body = render("shell-job-report/body.mustache", args)
    logger.debug("email message: %s\n%s", subject, body)

    task = email_pb2.Task(subject=subject.strip(),
                          body=email_pb2.Task.Body(html=True, content=body))
    for it in cc:
        task.cc.append(email_pb2.Task.Address(email=it))
    for it in bcc:
        task.bcc.append(email_pb2.Task.Address(email=it))

    return task


def _acknowledge_and_publish(channel, delivery_tag, queue, task: email_pb2.Task):
    if not channel.is_open:
        logger.error("channel was closed")
        return
    properties = protobuf_message_properties()
    logger.debug("publish email-send message to %s://%s/%s", queue,
                 properties.content_type, properties.message_id)
    channel.basic_publish(exchange='', routing_key=queue,
                          body=task.SerializeToString(), properties=properties)
    channel.basic_ack(delivery_tag=delivery_tag)


def _handle_message(connection, channel, delivery_tag, working_dir, email_queue, cc, bcc, db_url, message):
    try:
        launched_at = datetime.now(UTC)

        task = hydrangea_pb2.Task()
        task.ParseFromString(message)
        result, elapsed = _execute(working_dir, task)
        logger.debug("%d\n%s\n%s", result.returncode,
                     result.stdout, result.stderr)
        email_task = _build_email_task(cc, bcc, {
            "task": task,
            "status_code": result.returncode,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "launched_at": to_timestamp(launched_at),
            "elapsed": isodate.duration_isoformat(timedelta(microseconds=elapsed / 1_000)),
        })
        logger.debug("save resulting for %s", task.sn)
        with psycopg.connect(db_url) as db:
            with db.cursor() as cur:
                cur.execute(
                    "UPDATE hydrangea_job_logs SET launched_at=%s,elapsed=%s,status_code=%s,stdout=%s,stderr=%s,updated_at=CURRENT_TIMESTAMP WHERE sn=%s",
                    (launched_at, elapsed,  result.returncode,
                        result.stdout, result.stderr, task.sn)
                )

        cb = functools.partial(_acknowledge_and_publish,
                               channel, delivery_tag, email_queue, email_task)
        connection.add_callback_threadsafe(cb)
    except Exception:
        logger.exception("execute shell-run task")
        cb = functools.partial(
            channel.basic_nack, delivery_tag=delivery_tag, requeue=False)
        connection.add_callback_threadsafe(cb)


def _on_message(channel, method, properties, body, args):
    logger.info("receive message(reply to %s) %s %s %s",
                properties.reply_to, properties.message_id, properties.content_type, properties.type)
    connection, executor, working_dir, cc, bcc, db_url = args
    delivery_tag = method.delivery_tag
    executor.submit(_handle_message, connection, channel,
                    delivery_tag, working_dir, properties.reply_to, cc, bcc, db_url, body)
