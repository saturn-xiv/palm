import logging
from datetime import UTC
import pickle

from systemd import journal


from daisy.protocols import lavender_pb2, to_timestamp

logger = logging.getLogger(__name__)

INDEX_NAME = "lavender.graphql.logging.systemd.unit.item"


def launch(stub, name, db):
    logger.info("load systemd unit %s", name)
    try:
        stub.Systemd(_load_logs_for_unit(db, name))
    except Exception:
        logger.exception("fetch systemd logs")


def _load_logs_for_unit(db, name):
    key = f"systemd.{name}.last-fetch"

    reader = journal.Reader()
    reader.log_level(journal.LOG_INFO)

    if key in db:
        since = pickle.loads(db[key])
        logger.debug("fetch systemd logs for %s since %s", name, since)
        reader.seek_realtime(since)
    else:
        logger.debug("fetch systemd logs for %s since last-boot", name)
        reader.this_boot()

    reader.add_match(_SYSTEMD_UNIT=name)

    for entry in reader:
        cur = entry.get('__REALTIME_TIMESTAMP')
        yield lavender_pb2.SystemdRequest(
            host=entry.get('_HOSTNAME'),
            name=entry.get('_SYSTEMD_UNIT'),
            created_at=to_timestamp(cur),
            priority=int(entry.get('PRIORITY', '0')),
            message=entry.get('MESSAGE', '')
        )
        db[key] = pickle.dumps(cur)
