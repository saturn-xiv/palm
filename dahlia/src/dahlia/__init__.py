import os
import logging
import argparse
import tomllib
import signal
import threading
from concurrent import futures
from time import sleep
from importlib.metadata import version

import grpc
from grpc_reflection.v1alpha import reflection
from grpc_health.v1 import health, health_pb2, health_pb2_grpc


from . import rbac
from .rbac.server import Server as RbacServer
from dahlia.protocols import rbac_pb2_grpc, rbac_pb2

logger = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser(
        description="A rbac service(gRPC).", formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument(
        '-c', '--config', default='config.toml', help="config file path")
    parser.add_argument('-s', '--tls',
                        action='store_true', help='with mutal-TLS mode')
    parser.add_argument('-p', '--port', type=int,
                        default=8080, help="port to listen")
    parser.add_argument('-w', '--workers', type=int,
                        default=os.cpu_count(), help='max of workers')
    parser.add_argument('-d', '--debug',
                        action='store_true', help='run on debug mode')
    parser.add_argument('-v', '--verbose',
                        action='version', version=version(__package__))
    args = parser.parse_args()
    logging.basicConfig(
        format='%(asctime)s %(levelname).1s %(message)s', level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("running on debug mode")

    logger.debug("load configuration from %s", args.config)
    with open(args.config, "rb") as file:
        config = tomllib.load(file)
    launch_grpc_server(config, args.port, args.workers, args.tls)


def launch_grpc_server(config, port, workers, tls):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=workers))

    enforcer = rbac.open_enforcer(config['postgresql'], config['rabbitmq'])
    rbac_pb2_grpc.add_EnforcerServicer_to_server(RbacServer(enforcer), server)

    reflection.enable_server_reflection((
        rbac_pb2.DESCRIPTOR.services_by_name["Enforcer"].full_name,
        reflection.SERVICE_NAME,
    ), server)

    health_servicer = health.HealthServicer(
        experimental_non_blocking=True,
        experimental_thread_pool=futures.ThreadPoolExecutor(
            max_workers=workers),
    )
    health_pb2_grpc.add_HealthServicer_to_server(health_servicer, server)

    toggle_health_status_thread = threading.Thread(
        target=toggle_grpc_health,
        args=(health_servicer, "*"),
        daemon=True,
    )
    toggle_health_status_thread.start()

    addr = f"0.0.0.0:{port}"
    if tls:
        with open("server.key", "rb") as f:
            server_private_key = f.read()
        with open("server.crt", "rb") as f:
            server_cert_chain = f.read()
        with open("ca.crt", "rb") as f:
            ca_cert = f.read()
        credentials = grpc.ssl_server_credentials(
            private_key_certificate_chain_pairs=[
                (server_private_key, server_cert_chain)],
            root_certificates=ca_cert,
            require_client_auth=True
        )
        logger.info(
            "start gRPC server on tcp://%s with %d workers mTLS mode", addr, workers)
        server.add_secure_port(addr, credentials)
    else:
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


def toggle_grpc_health(health_servicer: health.HealthServicer, service: str):
    while True:
        health_servicer.set(service, health_pb2.HealthCheckResponse.SERVING)
        sleep(5)
