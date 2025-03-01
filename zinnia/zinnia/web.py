import os
import logging

from flask import Flask
import gunicorn.app.base

from . import bbs, cms, questionnaire, bookkeeper, HTTP_STATUS_NOT_FOUND, HTTP_STATUS_INTERNAL_SERVER_ERROR


def create_app(debug, config_file):
    app = Flask(__name__, instance_relative_config=True)

    app.debug = debug
    app.logger.setLevel(logging.DEBUG if debug else logging.INFO)
    app.logger.info("load configuration from %s", config_file)
    # app.config.from_prefixed_env()

    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass

    app.register_blueprint(bbs.router)
    app.register_blueprint(questionnaire.router)
    app.register_blueprint(bookkeeper.router)

    app.add_url_rule('/pages/<slug>', view_func=cms.show_page_by_slug)
    app.add_url_rule('/<lang>/rss.xml', view_func=cms.rss_xml)
    app.add_url_rule('/<lang>/sitemap.xml', view_func=cms.sitemap_xml_by_lang)
    app.add_url_rule('/sitemap.xml', view_func=cms.sitemap_xml)
    app.add_url_rule('/robots.txt', view_func=cms.robots_txt)
    app.add_url_rule('/nginx.conf', view_func=cms.nginx_conf)
    app.add_url_rule('/', view_func=cms.home)

    app.register_error_handler(HTTP_STATUS_NOT_FOUND, cms.not_found)
    app.register_error_handler(
        HTTP_STATUS_INTERNAL_SERVER_ERROR, cms.internal_server)

    return app


class StandaloneApplication(gunicorn.app.base.BaseApplication):

    def __init__(self, app, options=None):
        self.options = options or {}
        self.application = app
        super().__init__()

    def load_config(self):
        config = {key: value for key, value in self.options.items()
                  if key in self.cfg.settings and value is not None}
        for key, value in config.items():
            self.cfg.set(key.lower(), value)

    def load(self):
        return self.application
