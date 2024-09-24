import logging
import tomllib


logger = logging.getLogger(__name__)


def launch(args):
    logger.debug("fetch udp://%s:%d" % (args.host, args.port))
    with open(args.config, "rb") as config_file:
        config = tomllib.load(config_file)
