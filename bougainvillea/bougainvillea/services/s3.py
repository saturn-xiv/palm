import logging

from google.protobuf import empty_pb2 as google_empty_pb2

from palm.s3.v1 import s3_pb2_grpc, s3_pb2

logger = logging.getLogger(__name__)


class Service(s3_pb2_grpc.S3Servicer):
    def CreateBucket(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def ListBucket(self, request, context):
        # TODO
        logger.debug("list buckets")
        return s3_pb2.ListBucketResponse()

    def PresignedPutObject(self, request, context):
        # TODO
        return s3_pb2.PresignedPutObjectResponse()

    def ObjectPermanentUrl(self, request, context):
        # TODO
        return s3_pb2.UrlResponse()

    def ObjectPresignedUrl(self, request, context):
        # TODO
        return s3_pb2.UrlResponse()

    def RemoveObject(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def RemoveBucket(self, request, context):
        # TODO
        return google_empty_pb2.Empty()
