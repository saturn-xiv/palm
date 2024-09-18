# -*- coding: utf-8 -*-

import logging
import tomllib
import argparse

from bougainvillea.services import launch as start_rpc_server

logger = logging.getLogger(__name__)


def launch_rpc_server(args):
    # TODO
    logger.debug("load tls from files: %s, %s, %s" %
                 (args.ca_file, args.cert_file, args.key_file))
    start_rpc_server("d", "q", "s", args.port,
                     (open(args.ca_file, 'rb').read(), open(args.cert_file, 'rb').read(), open(args.key_file, 'rb').read()))


def launch_queue_consumer(args):
    logger.info("start a queue(%s) consumer(%s) for %s" %
                (args.queue, args.consumer, args.task))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='bougainvillea',
        description='A collection of minio/casbin services and pandoc/texlive/email/sms workers',
        epilog='https://github.com/saturn-xiv/palm')
    parser.add_argument('-c', '--config', default="config.toml",
                        help="load configuration")
    parser.add_argument(
        '-d', '--debug', help="run on debug mode", action='store_true')
    parser.add_argument('-v', '--verbose',
                        help="print version", action='version', version='2024.9.18')

    subparsers = parser.add_subparsers(help='sub-commands help')

    cmd_rpc_server = subparsers.add_parser(
        'rpc-server', help='start a gRPC server')
    cmd_rpc_server.add_argument('-p', '--port', type=int,
                                required=True, help='port to listen')
    cmd_rpc_server.add_argument('--ca-file', default='ca.crt')
    cmd_rpc_server.add_argument('--cert-file', default='server.crt')
    cmd_rpc_server.add_argument('--key-file', default='server.key')
    cmd_rpc_server.set_defaults(func=launch_rpc_server)

    cmd_queue_consumer = subparsers.add_parser(
        'queue-consumer', help='start a queue consumer')
    cmd_queue_consumer.add_argument(
        '-q', '--queue', required=True, help='queue name')
    cmd_queue_consumer.add_argument(
        '-n', '--consumer', required=True, help='consumer name')
    cmd_queue_consumer.add_argument(
        '-t', '--task', required=True, help='task name')
    cmd_queue_consumer.set_defaults(func=launch_queue_consumer)

    args = parser.parse_args()
    logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',
                        level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("run on debug mode")
    logger.info("load configuration from %s" % args.config)
    args.func(args)
    # with open("pyproject.toml", "rb") as fd:
    #     config = tomllib.load(fd)
