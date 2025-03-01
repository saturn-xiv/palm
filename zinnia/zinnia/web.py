import os
import logging

from flask import Flask

from . import bbs, cms, questionnaire, bookkeeper

app = Flask(__name__)


def create_app(debug, config_file):
    app = Flask(__name__, instance_relative_config=True)
    app.logger.info("aaa")

    app.logger.setLevel(logging.DEBUG if debug else logging.INFO)
    if debug:
        app.logger.debug("run on debug mode")
    else:
        app.logger.setLevel(logging.INFO)

    app.logger.info("load configuration from %s", config_file)
    app.config.from_prefixed_env()

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
    app.add_url_rule('/', view_func=cms.home)

    app.register_error_handler(404, cms.not_found)
    app.register_error_handler(500, cms.internal_server)

    return app
