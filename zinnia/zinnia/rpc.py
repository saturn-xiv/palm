import logging
import threading
from time import sleep
from concurrent import futures

import grpc
from grpc_reflection.v1alpha import reflection
from grpc_health.v1 import health, health_pb2, health_pb2_grpc


from .protocols import zinnia_pb2_grpc, zinnia_pb2
from .cms.rpc import CmsServer

logger = logging.getLogger(__name__)


def launch(host, port, max_workers):
    addr = '%s:%s' % (host, port)
    server = grpc.server(futures.ThreadPoolExecutor(max_workers))
    zinnia_pb2_grpc.add_CmsServicer_to_server(CmsServer(), server)
    SERVICE_NAMES = (
        zinnia_pb2.DESCRIPTOR.services_by_name["Cms"].full_name,
        reflection.SERVICE_NAME,
    )
    reflection.enable_server_reflection(SERVICE_NAMES, server)
    server.add_insecure_port(addr)
    _configure_health_server(server, max_workers)
    server.start()
    logger.info(
        "rpc server started, listening on tcp://%s with %d workers", addr, max_workers)
    server.wait_for_termination()


def _health_server(health_servicer: health.HealthServicer, service: str):
    while True:
        health_servicer.set(service, health_pb2.HealthCheckResponse.SERVING)
        sleep(5)


def _configure_health_server(server: grpc.Server, max_workers: int):
    health_servicer = health.HealthServicer(
        experimental_non_blocking=True,
        experimental_thread_pool=futures.ThreadPoolExecutor(max_workers),
    )
    health_pb2_grpc.add_HealthServicer_to_server(health_servicer, server)

    toggle_health_status_thread = threading.Thread(
        target=_health_server,
        args=(health_servicer, "palm.zinnia.v1.Cms"),
        daemon=True,
    )
    toggle_health_status_thread.start()
