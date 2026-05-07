from pathlib import Path
import logging
import os

import casbin
import sqlalchemy_adapter
import casbin_rabbitmq_watcher
from google.protobuf.empty_pb2 import Empty

from dahlia.protocols import rbac_pb2_grpc

logger = logging.getLogger(__name__)


class Server(rbac_pb2_grpc.EnforcerServicer):
    def __init__(self, enforcer):
        self.enforcer = enforcer

    def HasPermission(self, request, context):
        # TODO
        return Empty()


def update_callback_func(msg):
    logging.debug("receive casbin watcher message: %s", msg)


# https://docs.sqlalchemy.org/en/20/dialects/postgresql.html#dialect-postgresql-psycopg-connect
# https://docs.sqlalchemy.org/en/20/dialects/mysql.html#module-sqlalchemy.dialects.mysql.mariadbconnector
# https://docs.sqlalchemy.org/en/20/dialects/sqlite.html#module-sqlalchemy.dialects.sqlite.pysqlite
def open_enforcer(db, rabbitmq):
    logger.debug("open sqlalchemy adapter postgresql://%s@%s:%d/%s",
                 db['user'], db['host'], db['port'], db['db-name'])
    adapter = sqlalchemy_adapter.Adapter(
        f"postgresql+psycopg://{db['user']}:{db['password']}@{db['host']}:{db['port']}/{db['db-name']}?sslmode=disable")

    logger.debug("open rabbitmq watcher %s@%s:%d/%s",
                 rabbitmq['user'], rabbitmq['host'], rabbitmq['port'], rabbitmq['virtual-host'])
    watcher = casbin_rabbitmq_watcher.new_watcher(host=rabbitmq['host'], port=rabbitmq['port'],
                                                  username=rabbitmq['user'], password=rabbitmq['password'], virtual_host=rabbitmq['virtual-host'])
    watcher.set_update_callback(update_callback_func)

    # https://casbin.apache.org/docs/supported-models/
    model_file = Path(__file__).parent / 'rbac_model.conf'
    logger.debug('load casbin model from %s', model_file)
    enforcer = casbin.Enforcer(os.path.abspath(model_file), adapter)
    enforcer.set_watcher(watcher)
    return enforcer
