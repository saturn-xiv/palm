import logging
import argparse
import tomllib

from . import rbac

logger = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser(description="A rbac service(gRPC).")
    parser.add_argument('-c', '--config', default='config.toml')
    parser.add_argument('-p', '--port', type=int, default=8080)
    parser.add_argument('-d', '--debug',
                        action='store_true', help='run on debug mode')
    parser.add_argument('-v', '--verbose',
                        action='version', version='2026.5.7')
    args = parser.parse_args()
    logging.basicConfig(
        format='%(asctime)s %(levelname).1s %(message)s', level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("running on debug mode")

    logger.debug("load configuration from %s", args.config)
    with open(args.config, "rb") as file:
        config = tomllib.load(file)
        launch_grpc_server(config, args.port)


def launch_grpc_server(config, port):
    enforcer = rbac.open_enforcer(config['postgresql'], config['rabbitmq'])
    logger.info("start gRPC server on tcp://0.0.0.0:%d", port)
