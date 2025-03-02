from concurrent import futures
import logging

import grpc

from .protocols import zinnia_pb2_grpc
from .cms.rpc import CmsServer

logger = logging.getLogger(__name__)


def launch(host, port, max_workers):
    addr = '%s:%s' % (host, port)
    server = grpc.server(futures.ThreadPoolExecutor(max_workers))
    zinnia_pb2_grpc.add_CmsServicer_to_server(CmsServer(), server)
    server.add_insecure_port(addr)
    server.start()
    logger.info("rpc server started, listening on tcp://%s", addr)
    server.wait_for_termination()
