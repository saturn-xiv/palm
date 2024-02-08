import logging

from email.message import EmailMessage
from email.mime.base import MIMEBase
from email.utils import formataddr
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from email import encoders

from pumpkin import lily_pb2
from . import detect_mime_type

SEND_EMAIL_JOB = "send-email"


def create_email_send_queue_callback(client):
    def it(ch, method, properties, body):
        logging.info("receive message %s", properties.message_id)
        _handle_email_send_message(body, client)
        ch.basic_ack(delivery_tag=method.delivery_tag)
    return it


def _handle_email_send_message(message, client):
    task = lily_pb2.EmailSendTask()
    task.ParseFromString(message)
    logging.info("send email %s to %s<%s>", task.to.subject,
                 task.to.name, task.to.email)
    to = formataddr((task.to.name, task.to.email))

    msg = MIMEMultipart()
    msg["Subject"] = task.subject
    msg["Cc"] = list(map(lambda x: formataddr((x.name, x.email)), task.cc))
    msg["Bcc"] = list(map(lambda x: formataddr((x.name, x.email)), task.bcc))

    if msg.body.html:
        body = MIMEText(msg.body.payload, "html")
        msg.attach(body)
    else:
        body = MIMEText(msg.body.payload)
        msg.attach(body)

    for it in msg.attachments:
        mt, st = detect_mime_type(it.title)
        part = MIMEBase(mt, st)
        part.set_payload(it.payload)
        encoders.encode_base64(part)
        part.add_header("Content-Disposition",
                        "attachment; filename='%s'" % it.title)
        msg.attach(part)
    client.send(to, msg)
