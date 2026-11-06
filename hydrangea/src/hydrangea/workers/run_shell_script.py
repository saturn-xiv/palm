import logging
import signal

from ..etc import rabbitmq as open_rabbitmq, is_stopped

logger = logging.getLogger(__name__)


def launch(config, queue):
    if is_stopped():
        return
    channel = open_rabbitmq(config)
    channel.queue_declare(queue=queue, durable=True,
                          arguments={})
    channel.basic_consume(
        queue=queue, on_message_callback=callback, auto_ack=False)

    def handle_shutdown(signum, frame):
        logger.warning(
            "received signal %d, waiting for shutdown...", signum)
        channel.stop_consuming()

    signal.signal(signal.SIGTERM, handle_shutdown)
    signal.signal(signal.SIGINT, handle_shutdown)

    logging.info("waiting for messages. To exit press CTRL+C")
    channel.start_consuming()
    logger.info('exited')


def callback(channel, method, properties, body):
    logger.info("receive message %s %s",
                properties.message_id, properties.content_type)
    channel.basic_ack(delivery_tag=method.delivery_tag)
    # TODO
    if is_stopped():
        channel.stop_consuming()
