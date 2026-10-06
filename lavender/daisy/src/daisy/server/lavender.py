import logging

from grpc import StatusCode

from daisy.protocols import lavender_pb2_grpc, lavender_pb2

logger = logging.getLogger(__name__)


class Reporter(lavender_pb2_grpc.ReporterServicer):
    def __init__(self, db, namespace):
        self.db = db
        self.namespace = namespace
        # TODO init indexs

    def Systemd(self, request, context):
        logger.info("receive %d items", len(request.items))
        # TODO
        return lavender_pb2.Empty()
