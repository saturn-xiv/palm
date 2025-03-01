import sys
import logging

logger = logging.getLogger(__name__)


def main():
    # filename='zinnia.log',
    logging.basicConfig(level=logging.DEBUG)
    logger.info("start")


if __name__ == "__main__":
    main()
