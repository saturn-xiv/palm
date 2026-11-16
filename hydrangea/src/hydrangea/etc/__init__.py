import logging
from pathlib import Path

from opensearchpy import OpenSearch
import pika

logger = logging.getLogger(__name__)


# https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING-KEYWORD-VALUE
def postgresql(config):
    db_name = config['db-name']
    user = config['user']
    host = config['host']
    port = config['port']
    logger.debug("open PostgreSql %s@%s:%d/%s", user, host, port, db_name)
    return f"dbname={db_name} user={user} password={config['password']} host={host} port={port} sslmode=disable"


def opensearch(config):
    host = config['host']
    port = config['port']

    logging.debug("open OpenSearch http://%s:%d", host, port)
    return OpenSearch(hosts=[{'host': host, 'port': port}], http_compress=True, use_ssl=False)


def rabbitmq(config):
    host = config['host']
    port = config['port']
    user = config['user']
    password = config['password']
    virtual_host = config['virtual-host']

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
