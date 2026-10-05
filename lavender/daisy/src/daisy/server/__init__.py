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
from .lavender import StorageServer as LavenderStorageServer

logger = logging.getLogger(__name__)


def launch(config, port, workers):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=workers))

    db = open_opensearch(config['opensearch']['host'],
                         config['opensearch']['port'])
    lavender_pb2_grpc.add_StorageServicer_to_server(
        LavenderStorageServer(db, config['opensearch']['namespace']), server)

    reflection.enable_server_reflection((
        lavender_pb2.DESCRIPTOR.services_by_name["Storage"].full_name,
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
