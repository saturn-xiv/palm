import logging
import json
import uuid
import os.path
import smtplib
import pathlib


from email.utils import formataddr
from datetime import timedelta, datetime


import pika
import psycopg
from minio import Minio
from minio.versioningconfig import VersioningConfig as MinioVersioningConfig
from minio.commonconfig import ENABLED as MinioEnabled, Tags as MinioTags
from twilio.rest import Client as TwilioRestClient


VERSION = '2023.2.8'


def is_stopped():
    return os.path.isfile('.stop')


class MinioClient:
    def __init__(self, config):
        logging.debug("connect to minio %s", config['endpoint'])
        self.namespace = config['namespace']
        self.connection = Minio(
            config['endpoint'],
            access_key=config['access-key'],
            secret_key=config['secret-key'],
            secure=config['secure'])
        logging.debug('found buckets: %s', self.list_buckets())

    def put_object(self, bucket, name, data, length, content_type):
        logging.debug("try to upload(%s, %s, %s) with %d bytes",
                      bucket, name, content_type, length)
        result = self.connection.put_object(
            bucket, name, data, length, content_type=content_type)
        logging.info("uploaded %s, etag: %s, version-id: %s",
                     result.object_name, result.etag, result.version_id)

    def get_object_url(self, bucket, name, ttl=60*60*24*7):
        return self.connection.presigned_get_object(bucket, name, expires=timedelta(seconds=ttl))

    def set_object_tags(self, bucket, name, tags):
        tmp = MinioTags.new_object_tags()
        for k, v in tags:
            tmp[k] = v
        self.connection.set_object_tags(bucket, name, tmp)

    def bucket_exists(self, bucket, published=False):
        ok = self.connection.bucket_exists(bucket)
        if ok:
            return

        logging.warning("bucket %s isn't existed, try to create it")
        self.connection.make_bucket(bucket)
        self.connection.set_bucket_versioning(
            bucket, MinioVersioningConfig(MinioEnabled))

        if published:
            policy = {
                "Version": "2023-10-06",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {"AWS": "*"},
                        "Action": [
                            "s3:GetObject"
                        ],
                        "Resource": "arn:aws:s3:::%s/*" % bucket,
                    },
                ],
            }
            self.connection.set_bucket_policy(bucket, json.dumps(policy))

    def list_buckets(self):
        return list(map(lambda x: x.name, self.connection.list_buckets()))

    def current_bucket(self, published):
        return '-' .join([self.namespace, datetime.now().strftime("%Y"), ('o' if published else 'p')])

    def random_filename(ext=''):
        return str(uuid.uuid4())+ext


# https://pika.readthedocs.io/en/stable/modules/parameters.html
class RabbitMqClient:
    def __init__(self, config):
        credentials = pika.PlainCredentials(config['user'], config['password'])
        self.parameters = pika.ConnectionParameters(
            config['host'],
            config['port'],
            config['virtual-host'],
            credentials)

    def produce(self, queue, id, message):
        logging.info("publish message(%s) to (%s) with %d bytes",
                     id, queue, len(message))
        with pika.BlockingConnection(self.parameters) as con:
            ch = con.channel()
            ch.queue_declare(queue=queue, durable=True)
            ch.basic_publish(exchange='', routing_key=queue,
                             body=message, properties=pika.BasicProperties(message_id=id, delivery_mode=pika.spec.PERSISTENT_DELIVERY_MODE))

    def start_consuming(self, queue, callback):
        logging.info("start consumer for %s", queue)

        def handler(ch, method, properties, body):
            callback(ch, method, properties, body)
            if is_stopped():
                logging.warning("stop consumer")
                ch.stop_consuming()

        with pika.BlockingConnection(self.parameters) as con:
            ch = con.channel()
            ch.queue_declare(queue=queue, durable=True)
            ch.basic_qos(prefetch_count=1)
            ch.basic_consume(queue=queue, on_message_callback=handler)
            try:
                ch.start_consuming()
            except KeyboardInterrupt:
                logging.warning("quit consumer...")
                ch.stop_consuming()


class SmtpClient:
    def __init__(self, config):
        self.host = config['host']
        self.port = config['port']
        self.from_ = formataddr(
            (config['from']['name'], config['from']['email']))
        self.password = config['password']

    def send(self, to, message):
        with smtplib.SMTP_SSL(self.host, self.port) as con:
            con.login(self.from_[0], self.password)
            con.send_message(message, self.from_, to)

# https://www.twilio.com/docs/messaging/quickstart/python
# https://github.com/twilio/twilio-python
# https://www.twilio.com/docs/usage/webhooks/messaging-webhooks


class TwilioClient:
    def __init__(self, config):
        self.account_sid = config['account-sid']
        self.auth_token = config['auth-token']
        self.from_ = config['from']

    def send(self, to, body, status_callback=None):
        logging.info("send sms to %s: %s", to, body)
        client = TwilioRestClient(self.account_sid, self.auth_token)
        response = client.messages.create(
            body=body, to=to, from_=self.from_, status_callback=status_callback)
        logging.info("%s", response.sid)
        logging.debug("%s", json.dumps(response))

# -----------------------------------------------------------------------------


# https://www.postgresql.org/docs/current/libpq-connect.html
def postgresql_url(config):
    logging.debug('open postgresql://%s@%s:%d/%s',
                  config['user'], config['host'], config['port'], config['name'])
    url = 'host=%s port=%d user=%s password=%s dbname=%s sslmode=disable' % (
        config['host'], config['port'], config['user'], config['password'], config['name'])
    with psycopg.connect(url) as db:
        cur = db.cursor()
        cur.execute('SELECT VERSION(), CURRENT_TIMESTAMP')
        row = cur.fetchone()
        if row:
            logging.debug("%s %s", row[0], row[1])
    return url


def save_s3_file(s3, file, data, size, content_type):
    s3.bucket_exists(file.bucket, file.published)
    s3.put_object(file.bucket, file.name, data, size, content_type)
    tags = {'title': file.title}
    if file.has_owner:
        tags['owner'] = file.owner
    if file.has_ttl:
        tags['ttl'] = file.ttl.seconds
    s3.set_object_tags(file.bucket, file.name, tags)


# https://www.iana.org/assignments/media-types/media-types.xhtml
# https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
def detect_mime_type(title):
    ext = pathlib.Path(title).suffix
    if ext == ".pdf":
        return ("application", "pdf")
    if ext == ".mp4":
        return ("application", "mp4")
    if ext == ".doc" or ext == ".dot":
        return ("application", "vnd.msword")
    if ext == ".docx":
        return ("application", "vnd.openxmlformats-officedocument.wordprocessingml.document")
    if ext == ".doct":
        return ("application", "vnd.openxmlformats-officedocument.wordprocessingml.template")
    if ext == ".xls" or ext == ".xlt" or ext == ".xla":
        return ("application", "vnd.ms-excel")
    if ext == ".xlsx":
        return ("application", "vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    if ext == ".xltx":
        return ("application", "vnd.openxmlformats-officedocument.spreadsheetml.template")
    if ext == ".ppt" or ext == ".pot" or ext == ".pps" or ext == ".ppa":
        return ("application", "vnd.ms-powerpoint")
    if ext == ".pptx":
        return ("application", "vnd.openxmlformats-officedocument.presentationml.presentation")
    if ext == ".ppsx":
        return ("application", "vnd.openxmlformats-officedocument.presentationml.slideshow")
    if ext == ".potx":
        return ("application", "vnd.openxmlformats-officedocument.presentationml.template")
    if ext == ".mdb":
        return ("application", "vnd.ms-access")

    if ext == ".png":
        return ("image", "png")
    if ext == ".jpg" or ext == ".jpeg":
        return ("image", "jpeg")

    if ext == ".htm" or ext == ".html":
        return ("text", "html")
    if ext == ".txt":
        return ("text", "plain")
    if ext == ".css":
        return ("text", "css")
    if ext == ".js":
        return ("text", "javascript")

    logging.warn("unknown extation %s", ext)
    return ("application", "octet-stream")
