import logging
from datetime import datetime, timedelta, UTC
import socket

from systemd import journal


from daisy.protocols import lavender_pb2, to_timestamp

logger = logging.getLogger(__name__)


def logs_by_unit(stub, name):
    logger.info("load systemd unit %s", name)
    hostname = socket.gethostname()
    try:
        reader = journal.Reader()
        reader.log_level(journal.LOG_INFO)

        reader.this_boot()
        # since = datetime.now(UTC) - timedelta(minutes=15)
        # logger.debug("fetch logs for %s since %s", name, since)
        # reader.seek_realtime(since)

        reader.add_match(_SYSTEMD_UNIT="sshd.service")

        request = lavender_pb2.SystemdRequest(items=[])
        for entry in reader:
            request.items.append(lavender_pb2.SystemdRequest.Item(
                host=hostname,
                unit=entry.get('_SYSTEMD_UNIT'),
                timestamp=to_timestamp(entry.get('__REALTIME_TIMESTAMP')),
                priority=int(entry.get('PRIORITY', '0')),
                message=entry.get('MESSAGE', ''))
            )

        stub.Systemd(request)
    except Exception as e:
        logger.error("%s", e)
