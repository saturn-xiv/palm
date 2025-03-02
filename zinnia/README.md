# ZINNIA

## Usage

```bash
$ cd ~/workspace
$ ./saturn-xiv/palm/zinnia/docker/start.sh
> cd /workspace/saturn-xiv/palm/zinnia
> python3 -m venv $HOME/local/python3
> source $HOME/local/python3/bin/activate
> pip install -U pip setuptools
> pip install -e .
> pip list

# start web server
> FLASK_ASSETS_VERSION=20250301 gunicorn -k gevent -w 4 -b 127.0.0.1:8080 -p .pid 'zinnia.web:create_app(debug=True, config_file="config.toml")'
# stop web server
> kill -15 $(cat /tmp/zinnia.pid)
```

## Documents

- [Tink Cryptographic Library](https://developers.google.com/tink)
- [Casbin](https://casbin.org/docs/get-started)
- [Protobuf 3](https://protobuf.dev/programming-guides/proto3/)
- [Protocol Buffers Version Support](https://protobuf.dev/support/version-support/)
- [Sitemaps XML format](https://www.sitemaps.org/protocol.html)
- [Introduction to robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/intro)
- [IndexNow.org](https://www.indexnow.org/documentation)
- [Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml)

### Frontend

- [Chart.js](https://www.chartjs.org/docs/latest/getting-started/)
- [Fluent UI](https://react.fluentui.dev/?path=/docs/concepts-introduction--docs)

### Python

- [Flask](https://flask.palletsprojects.com/en/stable/quickstart/)
- [SQLAlchemy](https://docs.sqlalchemy.org/)
- [MariaDB](https://pypi.org/project/mariadb/)
- [PostgreSQL](https://www.psycopg.org/psycopg3/docs/basic/install.html)
- [Redis](https://redis.io/docs/latest/develop/clients/redis-py/)
- [RabbitMQ](https://www.rabbitmq.com/tutorials/tutorial-one-python)
- [MinIO](https://min.io/docs/minio/linux/developers/python/minio-py.html)
- [OpenSearch](https://opensearch.org/docs/latest/clients/python-high-level/)
- [Twilio](https://www.twilio.com/docs/libraries/reference/twilio-python/)
- [Graphql](https://docs.graphene-python.org/en/latest/)
- [Python Packaging User Guide](https://packaging.python.org/en/latest/overview/)
