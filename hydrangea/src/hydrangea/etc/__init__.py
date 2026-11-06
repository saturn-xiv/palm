import logging
from pathlib import Path

from opensearchpy import OpenSearch
import pika

logger = logging.getLogger(__name__)


def opensearch(config):
    host = config['opensearch']['host']
    port = config['opensearch']['port']

    logging.debug("open OpenSearch http://%s:%d", host, port)
    return OpenSearch(hosts=[{'host': host, 'port': port}], http_compress=True, use_ssl=False)


def rabbitmq(config):
    host = config['rabbitmq']['host']
    port = config['rabbitmq']['port']
    user = config['rabbitmq']['user']
    password = config['rabbitmq']['password']
    virtual_host = config['rabbitmq']['virtual-host']

    logger.debug("open RabbitMQ %s@%s:%d/%s", user, host, port, virtual_host)
    connection = pika.BlockingConnection(
        pika.ConnectionParameters(host=host, port=port, virtual_host=virtual_host, credentials=pika.PlainCredentials(user, password)))
    channel = connection.channel()
    return channel


def is_stopped():
    it = Path(".stop")
    ok = it.is_file()
    if ok:
        logging.warning("file %s exists, will be exited...", it)
    return ok
