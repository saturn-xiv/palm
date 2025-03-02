import importlib.metadata

from ..protocols import zinnia_pb2, zinnia_pb2_grpc


class CmsServer(zinnia_pb2_grpc.CmsServicer):
    def Measure(self, request, context):
        return zinnia_pb2.CmsMeasureResponse(version=importlib.metadata.version('zinnia'))
