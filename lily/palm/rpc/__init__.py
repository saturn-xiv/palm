import threading
import logging

from time import sleep
from concurrent import futures

import grpc
from grpc_health.v1 import health_pb2, health, health_pb2_grpc

from pumpkin import lily_pb2_grpc
from . import excel


class Server:
    def __init__(self, port, threads):
        self.addr = '0.0.0.0:%d' % (port)
        self.max_workers = threads

    def start(self):
        server = grpc.server(futures.ThreadPoolExecutor(
            max_workers=self.max_workers))

        lily_pb2_grpc.add_ExcelServicer_to_server(excel.Service(), server)

        Server._rpc_setup_health_thread(server)
        server.add_insecure_port(self.addr)
        server.start()
        logging.info(
            "Lily gRPC server started, listening on %s with %d threads", self.addr, self.max_workers)
        try:
            server.wait_for_termination()
        except KeyboardInterrupt:
            logging.warning('exited...')
            server.stop(0)

    def _rpc_health_checker(servicer, name):
        while True:
            servicer.set(name, health_pb2.HealthCheckResponse.SERVING)
            sleep(5)

    def _rpc_setup_health_thread(server):
        servicer = health.HealthServicer(
            experimental_non_blocking=True,
            experimental_thread_pool=futures.ThreadPoolExecutor(max_workers=2)
        )
        health_pb2_grpc.add_HealthServicer_to_server(servicer, server)
        health_checker_thread = threading.Thread(
            target=Server._rpc_health_checker,
            args=(servicer, 'palm.lily'),
            daemon=True
        )
        health_checker_thread.start()
