from concurrent import futures
import logging

import grpc
from grpc_reflection.v1alpha import reflection

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
    server.start()
    logger.info(
        "rpc server started, listening on tcp://%s with %d workers", addr, max_workers)
    server.wait_for_termination()
