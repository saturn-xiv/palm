import logging

logger = logging.getLogger(__name__)

def service(stub, name):
    logger.info("load systemd service %s", name)
