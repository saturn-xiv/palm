import signal
import logging
import threading
from concurrent import futures
from time import sleep

import grpc
from grpc_reflection.v1alpha import reflection
from grpc_health.v1 import health, health_pb2, health_pb2_grpc
from opensearchpy import OpenSearch

from daisy.protocols import lavender_pb2, lavender_pb2_grpc
from .lavender import Reporter as LavenderReporterServer

logger = logging.getLogger(__name__)


def launch(config, port, workers, tls):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=workers))

    client = open_opensearch(config['opensearch']['host'],
                             config['opensearch']['port'])
    reporter_service = LavenderReporterServer(
        client, config['opensearch']['namespace'])
    reporter_service.check_indices()
    lavender_pb2_grpc.add_ReporterServicer_to_server(reporter_service, server)

    reflection.enable_server_reflection((
        lavender_pb2.DESCRIPTOR.services_by_name["Reporter"].full_name,
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


def open_opensearch(host, port):
    return OpenSearch(hosts=[{'host': host, 'port': port}], http_compress=True, use_ssl=False)
