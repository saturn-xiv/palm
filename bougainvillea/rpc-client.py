# -*- coding: utf-8 -*-


import logging
import argparse

from google.protobuf import empty_pb2 as google_empty_pb2
import grpc

from palm.s3.v1 import s3_pb2, s3_pb2_grpc

logger = logging.getLogger(__name__)


def test_s3_list_buckets(channel):
    pass


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='bougainvillea',
        description='A bougainvillea rpc client',
        epilog='https://github.com/saturn-xiv/palm')

    parser.add_argument(
        '-d', '--debug', help="run on debug mode", action='store_true')
    parser.add_argument('-v', '--verbose',
                        help="print version", action='version', version='2024.9.18')
    parser.add_argument('-H', '--host',
                        default='127.0.0.1', help='rpc host')
    parser.add_argument('-p', '--port', type=int,
                        required=True, help='rpc port')

    args = parser.parse_args()
    logging.basicConfig(level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("run on debug mode")

    addr = "%s:%d" % (args.host, args.port)
    logger.info("connect to %s" % addr)
    with grpc.insecure_channel(addr) as channel:
        stub = s3_pb2_grpc.S3Stub(channel)
        response = stub.ListBucket(google_empty_pb2.Empty())
        logger.info("found buckets %s" % ",".join(
            list(map(lambda x: x.name, response.items))))
