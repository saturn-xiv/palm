import logging

import gunicorn.app.base
from twilio.twiml.messaging_response import MessagingResponse
from flask import Flask


app = Flask(__name__)


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


def launch_server(port, threads):
    addr = '%s:%s' % ('127.0.0.1', port)
    logging.info("listen on http://%s with %d workers", addr, threads)
    StandaloneApplication(app, {
        'bind': addr,
        'workers': threads,
    }).run()
