import logging
from concurrent.futures import ThreadPoolExecutor
import dbm.ndbm
from datetime import datetime,  UTC
import pickle

import grpc

from .protocols import lavender_pb2_grpc
from .models.systemd import logs_by_unit as systemd_logs_by_unit
from .models.kubernetes import logs_by_namespace as kubernetes_logs_by_namespace

logger = logging.getLogger(__name__)


def launch(config, tls, max_workers):
    with dbm.ndbm.open('worker.db', 'c') as db:
        db["startup"] = pickle.dumps(datetime.now(UTC))
        channel = open_channel(
            config["server"]["host"], config["server"]["port"], tls)
        stub = lavender_pb2_grpc.ReporterStub(channel)

        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            for name in config["systemd"]["units"]:
                executor.submit(systemd_logs_by_unit, stub, name, db)
            for name in config["kubernetes"]["namespaces"]:
                executor.submit(kubernetes_logs_by_namespace, stub, name, db)

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
