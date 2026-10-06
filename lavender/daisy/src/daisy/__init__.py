import os
import logging
import argparse
import tomllib
from importlib.metadata import version


from .server import launch as launch_grpc_server
from .worker import launch as launch_worker

logger = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser(
        description="gRPC services for Lavender.", formatter_class=argparse.ArgumentDefaultsHelpFormatter)

    parser.add_argument(
        '-c', '--config', default='config.toml', help="config file path")
    parser.add_argument('-d', '--debug',
                        action='store_true', help='run on debug mode')
    parser.add_argument('-s', '--tls',
                        action='store_true', help='with mutal-TLS mode')
    parser.add_argument('-v', '--verbose',
                        action='version', version=version(__package__))

    sub_parsers = parser.add_subparsers(
        dest="command", required=True, help="Available sub-commands")
    parser_server = sub_parsers.add_parser(
        "server", help="Launch a gRPC server")
    parser_server.add_argument('-p', '--port', type=int,
                               default=8080, help="port to listen")
    parser_server.add_argument('-w', '--workers', type=int,
                               default=os.cpu_count(), help='max of workers')

    parser_worker = sub_parsers.add_parser(
        "worker", help="Launch a clawer worker")
    parser_worker.add_argument(
        '-c', '--config', default='config.toml', help="config file path")
    parser_worker.add_argument('-w', '--workers', type=int,
                               default=os.cpu_count(), help='max of workers')

    args = parser.parse_args()
    logging.basicConfig(
        format='%(asctime)s %(levelname).1s %(message)s', level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("running on debug mode")

    logger.debug("load configuration from %s", args.config)
    with open(args.config, "rb") as file:
        config = tomllib.load(file)
    if args.command == "server":
        launch_grpc_server(config, args.port, args.workers, args.tls)
    elif args.command == "worker":
        launch_worker(config, args.tls, args.workers)
