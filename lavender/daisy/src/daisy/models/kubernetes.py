import logging

logger = logging.getLogger(__name__)


def logs_by_namespace(stub, name):
    logger.info("load kubernetes namespace %s", name)
