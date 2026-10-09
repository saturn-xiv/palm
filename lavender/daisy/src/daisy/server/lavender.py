import logging
from datetime import datetime

from grpc import StatusCode
import numpy

from daisy.models.http import INDEX_NAME as HTTP_INDEX_NAME
from daisy.models.systemd import INDEX_NAME as SYSTEMD_INDEX_NAME
from daisy.models.kubernetes import INDEX_NAME as KUBERNETES_INDEX_NAME
from daisy.protocols import lavender_pb2_grpc, lavender_pb2


logger = logging.getLogger(__name__)


class Reporter(lavender_pb2_grpc.ReporterServicer):
    def __init__(self, client, namespace):
        self.client = client
        self.namespace = namespace

    def Systemd(self, request_iterator, context):
        logger.debug("start to receiving systemd data stream")
        chunk_count = 0

        for req in request_iterator:
            # logger.debug("receive chunk(%s): %s %s %s",
            #              chunk_count, req.unit, req.created_at, req.message)
            doc = {'host': req.host, 'unit': req.unit, 'priority': req.priority,
                   'message': req.message, 'created_at': from_timestamp(req.created_at)}
            res = self.client.index(index=self._index(
                SYSTEMD_INDEX_NAME), body=doc, refresh=True)
            logger.debug("index systemd-log %s", res['_id'])

            chunk_count += 1

        logger.debug(
            "finished receiving systemd stream, total %d chunks", chunk_count)
        return lavender_pb2.SystemdResponse(chunk_count=chunk_count)

    def Kubernetes(self, request_iterator, context):
        logger.debug("start to receiving kubernetes data stream")
        chunk_count = 0

        for req in request_iterator:
            # logger.debug("receive chunk(%s): %s %s %s",
            #              chunk_count, req.pod, req.node, req.container)
            doc = {'node': req.node, 'pod': req.pod, 'container': req.container, 'owners': [
                {'kind': x.kind, 'name': x.name, 'uid': x.uid} for x in req.owners], 'message': req.message, 'created_at': from_timestamp(req.created_at)}
            res = self.client.index(index=self._index(
                SYSTEMD_INDEX_NAME), body=doc, refresh=True)
            logger.debug("index systemd-log %s", res['_id'])

            chunk_count += 1

        logger.debug(
            "finished receiving kubernetes stream, total %d chunks", chunk_count)
        return lavender_pb2.SystemdResponse(chunk_count=chunk_count)

    def Claw(self, request, context):
        if request.http:
            # logger.debug("receive http record(%s) from %s",
            #              request.http.url, getattr(request, 'from'))
            doc = {'from': getattr(request, 'from'), 'elapsed': from_duration(request.elapsed), 'created_at': from_timestamp(
                request.created_at), 'url': request.http.url, 'body': request.http.body}
            if request.http.HasField('content_type'):
                doc['content_type'] = request.http.content_type
            if request.http.HasField('status_code'):
                doc['status_code'] = request.http.status_code

            res = self.client.index(index=self._index(
                HTTP_INDEX_NAME), body=doc, refresh=True)
            logger.debug("index http-monitor %s", res['_id'])
        else:
            context.abort(StatusCode.UNIMPLEMENTED, 'not yet implemented')
            return

        return lavender_pb2.Empty()

    def _index(self, name: str) -> str:
        return f"{self.namespace}.{name}"

    def check_indices(self):
        index = self._index(HTTP_INDEX_NAME)
        if not self.client.indices.exists(index=index):
            logger.warning("create index %s", index)

        index = self._index(SYSTEMD_INDEX_NAME)
        if not self.client.indices.exists(index=index):
            logger.warning("create index %s", index)

        index = self._index(KUBERNETES_INDEX_NAME)
        if not self.client.indices.exists(index=index):
            logger.warning("create index %s", index)


def from_duration(v: lavender_pb2.Duration) -> int:
    return v.seconds*1_000_000_000+v.nanos


def from_timestamp(v: lavender_pb2.Timestamp) -> str:
    it = numpy.datetime64(v.seconds, 's')+numpy.timedelta64(v.nanos, 'ns')
    return numpy.datetime_as_string(it, timezone='UTC')
