import logging
import threading

import grpc

from .protocols import lavender_pb2, lavender_pb2_grpc
from .models.systemd import service as systemd_service
from .models.kubernetes import namespace as kubernetes_namespace

logger = logging.getLogger(__name__)


def launch(config, tls):
    channel = open_channel(
        config["server"]["host"], config["server"]["port"], tls)
    stub = lavender_pb2_grpc.StorageStub(channel)
    threads = []

    for name in config["systemd"]["services"]:
        t = threading.Thread(target=systemd_service, args=(stub, name))
        threads.append(t)
        t.start()

    for name in config["kubernetes"]["namespaces"]:
        t = threading.Thread(target=kubernetes_namespace, args=(stub, name))
        threads.append(t)
        t.start()

    for it in threads:
        it.join()
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
