import logging


import gunicorn.app.base
from flask import Flask, make_response, request
from twilio.twiml.messaging_response import MessagingResponse

app = Flask(__name__)


# https://www.twilio.com/docs/messaging/tutorials/how-to-receive-and-reply/python
@app.route("/twilio/callback/incoming-text-message", methods=['POST'])
def sms_reply():
    body = request.values.get('Body', None)
    logging.info("receive sms message %s", body)
    msg = MessagingResponse()
    msg.message("Succeed!")

    res = make_response(str(msg))
    res.headers['Content-type'] = 'application/xml; charset=utf-8'
    return res

# https://www.twilio.com/docs/messaging/guides/track-outbound-message-status


@app.route("/twilio/callback/message-status", methods=['POST'])
def incoming_sms():
    message_sid = request.values.get('MessageSid', None)
    message_status = request.values.get('MessageStatus', None)
    logging.info('SID: %s, Status: %s', message_sid, message_status)

    res = make_response("", 204)
    res.headers['Content-type'] = 'text/plain; charset=utf-8'
    return res


class StandaloneApplication(gunicorn.app.base.BaseApplication):
    def __init__(self, app, options=None):
        self.options = options or {}
        self.application = app
        super(StandaloneApplication, self).__init__()

    def load_config(self):
        config = {key: value for key, value in self.options.items()
                  if key in self.cfg.settings and value is not None}
        for key, value in config.items():
            self.cfg.set(key.lower(), value)

    def load(self):
        return self.application

# https://docs.gunicorn.org/en/stable/custom.html


def launch_http_server(port, threads):
    addr = '%s:%s' % ('127.0.0.1', port)
    logging.info("listen on http://%s with %d workers", addr, threads)
    StandaloneApplication(app, {
        'bind': addr,
        'workers': threads,
    }).run()
