import logging

logger = logging.getLogger(__name__)

def namespace(stub, name):
    logger.info("load kubernetes namespace %s", name)
