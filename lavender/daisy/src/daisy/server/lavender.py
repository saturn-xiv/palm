import logging

from grpc import StatusCode

from daisy.protocols import lavender_pb2_grpc, lavender_pb2

logger = logging.getLogger(__name__)


class StorageServer(lavender_pb2_grpc.StorageServicer):
    def __init__(self, db, namespace):
        self.db = db
        self.namespace = namespace

    def Report(self, request, context):
        # TODO
        return lavender_pb2.Empty()
