import logging


from pumpkin import lily_pb2


JOB = "send-sms"

# https://www.twilio.com/docs/messaging/tutorials/how-to-receive-and-reply/python


def create_callback(client):
    def it(ch, method, properties, body):
        logging.info("receive message %s", properties.message_id)
        _handle_message(body, client)
        ch.basic_ack(delivery_tag=method.delivery_tag)
    return it


def _handle_message(message, client):
    task = lily_pb2.SmsSendTask()
    task.ParseFromString(message)
    for to in task.to:
        if task.has_callback:
            client.send(to, task.message, task.callback)
        else:
            client.send(to, task.message)
