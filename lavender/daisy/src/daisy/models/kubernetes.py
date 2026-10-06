import logging
from datetime import datetime, UTC
import pickle

from kubernetes import client, config, watch
import psutil
import numpy

from daisy.protocols import lavender_pb2, to_timestamp

logger = logging.getLogger(__name__)


def logs_by_namespace(stub, name, db):
    logger.info("load kubernetes namespace %s", name)
    try:
        config.load_kube_config()
        # config.load_incluster_config()
        stub.Kubernetes(_load_logs_for_namespace(db, name))
    except Exception:
        logger.exception("fetch kubernetes logs")


# https://k8s-python.readthedocs.io/en/stable/kubernetes.client.apis.html
# https://github.com/kubernetes-client/python/blob/master/kubernetes/docs/CoreV1Api.md
def _load_logs_for_namespace(db, namespace):
    v1 = client.CoreV1Api()
    logger.debug("load pods for namespace %s", namespace)
    pods = v1.list_namespaced_pod(namespace=namespace)
    if not pods.items:
        logger.warning("didn't have any pods")
        return
    for pod in pods.items:
        pod_name = pod.metadata.name
        containers = [x.name for x in pod.spec.containers]

        for container_name in containers:
            key = f"kubernetes.{namespace}.{pod_name}.{container_name}.last-fetch"
            since = datetime.fromtimestamp(psutil.boot_time(), tz=UTC)
            if key in db:
                since = pickle.loads(db[key])
            logger.debug("fetch kubernetes logs for %s@%s/%s since %s",
                         container_name, namespace, pod_name, since)
            # https://github.com/kubernetes-client/python/issues/1351
            # TODO since_time not support
            now = datetime.now(tz=UTC)
            res = v1.read_namespaced_pod_log(name=pod_name,
                                             namespace=namespace,
                                             container=container_name,
                                             timestamps=True,
                                             _preload_content=False,
                                             since_seconds=round(
                                                 (now-since).total_seconds()),
                                             )
            for line in watch.watch.iter_resp_lines(res):
                items = line.split(" ", maxsplit=1)
                if len(items) != 2:
                    logger.warning("ignore message: %s", line)
                    continue
                # UserWarning: no explicit representation of timezones available for np.datetime64
                cur = numpy.datetime64(items[0][:-1])
                yield lavender_pb2.KubernetesRequest(pod=pod_name, container=container_name, created_at=to_timestamp(cur), message=items[1])
                db[key] = pickle.dumps(cur.astype('datetime64[us]').item())
