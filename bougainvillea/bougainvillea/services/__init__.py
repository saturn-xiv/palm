import logging
from concurrent import futures

import grpc

from palm.rbac.v1 import rbac_pb2_grpc
from palm.s3.v1 import s3_pb2_grpc

from .rbac import Service as RbacService
from .s3 import Service as S3Service

logger = logging.getLogger(__name__)


def launch(db, queue, s3,  port, host='0.0.0.0', max_workers=16):
    addr = "%s:%d" % (host, port)
    logger.info("start gRPC server on http://%s" % addr)
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=max_workers))
    rbac_pb2_grpc.add_PolicyServicer_to_server(RbacService(), server)
    s3_pb2_grpc.add_S3Servicer_to_server(S3Service(), server)
    server.add_insecure_port(addr)
    server.start()
    server.wait_for_termination()
