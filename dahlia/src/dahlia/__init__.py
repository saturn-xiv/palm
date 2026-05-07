import logging
import argparse
import tomllib
from concurrent import futures
import signal

import grpc
from grpc_reflection.v1alpha import reflection

from . import rbac
from dahlia.protocols import rbac_pb2_grpc, rbac_pb2

logger = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser(description="A rbac service(gRPC).")
    parser.add_argument('-c', '--config', default='config.toml')
    parser.add_argument('-p', '--port', type=int, default=8080)
    parser.add_argument('-w', '--workers', type=int,
                        default=2, help='max of workers')
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
        launch_grpc_server(config, args.port, args.workers)


def launch_grpc_server(config, port, workers):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=workers))

    enforcer = rbac.open_enforcer(config['postgresql'], config['rabbitmq'])
    rbac_pb2_grpc.add_EnforcerServicer_to_server(rbac.Server(enforcer), server)

    reflection.enable_server_reflection((
        rbac_pb2.DESCRIPTOR.services_by_name["Enforcer"].full_name,
        reflection.SERVICE_NAME,
    ), server)

    addr = f"0.0.0.0:{port}"
    logger.info(
        "start gRPC server on tcp://%s with %d workers", addr, workers)
    server.add_insecure_port(addr)
    server.start()

    def handle_shutdown(signum, frame):
        logger.warning("received signal %d, waiting for shutdown...", signum)
        shutdown_event = server.stop(10)
        shutdown_event.wait(10)
    signal.signal(signal.SIGTERM, handle_shutdown)
    signal.signal(signal.SIGINT, handle_shutdown)

    server.wait_for_termination()
    logger.info('exited')
