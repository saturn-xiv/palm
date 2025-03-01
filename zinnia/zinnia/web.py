import os

from flask import Flask

app = Flask(__name__)


def create_app():
    app = Flask(__name__, instance_relative_config=True)
    app.config.from_mapping(
        SECRET_KEY='dev',
        DATABASE="db",
    )

    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass

    @app.route('/')
    def home():
        return 'Hello, World!'

    return app
