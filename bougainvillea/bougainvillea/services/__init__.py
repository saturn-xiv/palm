import logging
import threading
from concurrent import futures
from time import sleep

import grpc
from grpc_health.v1 import health
from grpc_health.v1 import health_pb2
from grpc_health.v1 import health_pb2_grpc

from palm.rbac.v1 import rbac_pb2_grpc
from palm.s3.v1 import s3_pb2_grpc

from .rbac import Service as RbacService
from .s3 import Service as S3Service

logger = logging.getLogger(__name__)


def _check_health(health_servicer: health.HealthServicer, service: str):
    logger.debug("run health check thread")
    while True:
        # TODO
        health_servicer.set(
            "palm.s3.v1.S3", health_pb2.HealthCheckResponse.SERVING)
        # TODO
        health_servicer.set(
            "palm.rbac.v1.Policy", health_pb2.HealthCheckResponse.SERVING)
        sleep(30)


def _configure_health_server(server: grpc.Server):
    health_servicer = health.HealthServicer(
        experimental_non_blocking=True,
        experimental_thread_pool=futures.ThreadPoolExecutor(max_workers=12),
    )
    health_pb2_grpc.add_HealthServicer_to_server(health_servicer, server)

    toggle_health_status_thread = threading.Thread(
        target=_check_health,
        args=(health_servicer, "helloworld.Greeter"),
        daemon=True,
    )
    toggle_health_status_thread.start()


def launch(db, queue, s3,  port, tls, host='0.0.0.0', max_workers=16):
    credentials = grpc.ssl_server_credentials(
        [(tls[2], tls[1])],
        root_certificates=tls[0],
        require_client_auth=True)
    addr = "%s:%d" % (host, port)
    logger.info("start gRPC server on http://%s" % addr)
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=max_workers))
    rbac_pb2_grpc.add_PolicyServicer_to_server(RbacService(), server)
    s3_pb2_grpc.add_S3Servicer_to_server(S3Service(), server)
    server.add_secure_port(addr, credentials)
    _configure_health_server(server)
    server.start()
    server.wait_for_termination()
