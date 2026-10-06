import logging
from datetime import datetime, UTC

from kubernetes import client, config
from kubernetes.client.rest import ApiException
import psutil
import pickle

from daisy.protocols import lavender_pb2, to_timestamp

logger = logging.getLogger(__name__)


def logs_by_namespace(stub, name, db):
    logger.info("load kubernetes namespace %s", name)
    stub.Kubernetes(_load_logs_for_namespace(db, name))


# https://k8s-python.readthedocs.io/en/stable/kubernetes.client.apis.html
# https://github.com/kubernetes-client/python/blob/master/kubernetes/docs/CoreV1Api.md
def _load_logs_for_namespace(db, namespace):
    try:
        config.load_kube_config()
        # config.load_incluster_config()
        v1 = client.CoreV1Api()
        logger.debug("load pods for namespace %s", namespace)
        pods = v1.list_namespaced_pod(namespace=namespace)
        if not pods.items:
            logger.warning("didn't have any pods")
            return
        for pod in pods.items:
            pod_name = pod.metadata.name
            containers = [x.name for x in pod.spec.containers]
            key = f"kubernetes.{namespace}.last-fetch"

            since = datetime.fromtimestamp(psutil.boot_time(), tz=UTC)
            if key in db:
                since = pickle.loads(db[key])
            logger.debug("fetch kubernetes logs for %s since %s",
                         namespace, since)

            for container_name in containers:
                logger.debug("fetch %s@%s", container_name, pod_name)
                logs = v1.read_namespaced_pod_log(
                    name=pod_name,
                    namespace=namespace,
                    container=container_name,
                    previous=True,
                    timestamps=True,
                    sinceTime=since
                )
                if logs:
                    for log in logs:
                        logger.debug("###: ", log)
                        yield lavender_pb2.KubernetesRequest(pod=pod_name, container=container_name, message=log)
                        # TODO
                        # db[key] = pickle.dumps(cur)
    except Exception:
        logger.exception("fetch kubernetes logs")
