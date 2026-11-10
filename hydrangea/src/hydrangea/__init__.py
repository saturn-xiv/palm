import os
import logging
import argparse
import tomllib
from importlib.metadata import version, metadata

from .rpc import launch as launch_rpc_server
from .workers import run_shell_script

logger = logging.getLogger(__name__)


def main():
    meta = metadata(__package__)
    parser = argparse.ArgumentParser(
        description=meta["Summary"], formatter_class=argparse.ArgumentDefaultsHelpFormatter)

    parser.add_argument(
        '-c', '--config', default='config.toml', help="config file path")
    parser.add_argument('-d', '--debug',
                        action='store_true', help='run on debug mode')
    parser.add_argument('-v', '--verbose',
                        action='version', version=version(__package__))

    sub_parsers = parser.add_subparsers(
        dest="command", required=True, help="Available sub-commands")

    parser_rpc = sub_parsers.add_parser("rpc", help="Launch a gRPC server")
    parser_rpc.add_argument('-p', '--port', type=int,
                            default=8080, help="port to listen")
    parser_rpc.add_argument('-w', '--workers', type=int,
                            default=os.cpu_count(), help='max of workers')
    parser_rpc.add_argument('-s', '--tls',
                            action='store_true', help='with mutual-TLS mode')
    parser_rpc.add_argument('--server-key', default='server.key')
    parser_rpc.add_argument('--server-crt', default='server.crt')
    parser_rpc.add_argument('--ca-crt', default='ca.crt')

    parser_run_shell_script = sub_parsers.add_parser(
        "run-shell-script", help="Launch a shell-script worker")
    parser_run_shell_script.add_argument(
        '-q', '--queue', required=True, help="queue name")

    args = parser.parse_args()
    logging.basicConfig(
        format='%(asctime)s %(levelname).1s %(message)s', level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("running on debug mode")

    logger.debug("load configuration from %s", args.config)
    with open(args.config, "rb") as file:
        config = tomllib.load(file)
    if args.command == "rpc":
        launch_rpc_server(config, args.port, args.workers, args.tls,
                          args.ca_crt, args.server_key, args.server_crt)
    elif args.command == "run-shell-script":
        run_shell_script.launch(config, args.queue)
