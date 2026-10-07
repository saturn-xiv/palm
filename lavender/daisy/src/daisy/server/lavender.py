import logging

from grpc import StatusCode

from daisy.protocols import lavender_pb2_grpc, lavender_pb2
from daisy.models.http import INDEX_NAME as HTTP_INDEX_NAME
from daisy.models.systemd import INDEX_NAME as SYSTEMD_INDEX_NAME
from daisy.models.kubernetes import INDEX_NAME as KUBERNETES_INDEX_NAME

logger = logging.getLogger(__name__)


class Reporter(lavender_pb2_grpc.ReporterServicer):
    def __init__(self, client, namespace):
        self.client = client
        self.namespace = namespace

    def Systemd(self, request_iterator, context):
        logger.debug("start to receiving systemd data stream")
        chunk_count = 0
        for req in request_iterator:
            chunk_count += 1
            # logger.debug("receive chunk(%s): %s %s %s",
            #              chunk_count, req.unit, req.created_at, req.message)
            # TODO

        logger.debug(
            "finished receiving systemd stream, total %d chunks", chunk_count)
        return lavender_pb2.SystemdResponse(chunk_count=chunk_count)

    def Kubernetes(self, request_iterator, context):
        logger.debug("start to receiving kubernetes data stream")
        chunk_count = 0
        for req in request_iterator:
            chunk_count += 1
            # logger.debug("receive chunk(%s): %s %s %s",
            #              chunk_count, req.unit, req.created_at, req.message)
            # TODO

        logger.debug(
            "finished receiving kubernetes stream, total %d chunks", chunk_count)
        return lavender_pb2.SystemdResponse(chunk_count=chunk_count)

    def Http(self, request, context):
        logger.debug("receive http record(%d) from %s",
                     len(request.items), request.host)

        return lavender_pb2.Empty()

    def __index(self, name: str) -> str:
        return f"{self.namespace}_{name}"

    def check_indices(self):
        index = self.__index(HTTP_INDEX_NAME)
        if not self.client.indices.exists(index=index):
            logger.warning("create index %s", index)

        index = self.__index(SYSTEMD_INDEX_NAME)
        if not self.client.indices.exists(index=index):
            logger.warning("create index %s", index)

        index = self.__index(KUBERNETES_INDEX_NAME)
        if not self.client.indices.exists(index=index):
            logger.warning("create index %s", index)
