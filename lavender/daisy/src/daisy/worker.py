import logging

import grpc

from .protocols import lavender_pb2, lavender_pb2_grpc

logger = logging.getLogger(__name__)


def launch(config, tls):
    channel = open_channel(
        config["server"]["host"], config["server"]["port"], tls)
    stub = lavender_pb2_grpc.StorageStub(channel)
    request = lavender_pb2.ReportRequest(items=[])
    stub.Report(request)
    logger.info("done.")


def open_channel(host, port, tls):
    addr = f"{host}:{port}"
    if tls:
        with open("ca.crt", "rb") as f:
            ca_cert = f.read()
        with open("client.key", "rb") as f:
            client_private_key = f.read()
        with open("client.crt", "rb") as f:
            client_cert_chain = f.read()
            credentials = grpc.ssl_channel_credentials(
                root_certificates=ca_cert,
                private_key=client_private_key,
                certificate_chain=client_cert_chain
            )
        logger.info("connect to tcp://%s with mTLS mode", addr)
        return grpc.secure_channel(addr, credentials)
    else:
        logger.info("connect to tcp://%s", addr)
        return grpc.insecure_channel(addr)
