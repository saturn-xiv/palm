
import logging
import select

from systemd import journal

from search import open_client as open_search_client

logger = logging.getLogger(__name__)


def launch(args):
    search_client = open_search_client(args.config)
    logger.debug("test index(%s) exists" % args.index_name)
    if not search_client.indices.exists(index=args.index_name):
        logger.warning("will crete index(%s)" % args.index_name)
        search_client.indices.create(
            args.index_name, body={'settings': {'index': {'number_of_shards': 4}}})

    reader = journal.Reader()
    reader.seek_tail()
    reader.get_previous()

    poller = select.poll()
    poller.register(reader, reader.get_events())

    while poller.poll():
        if reader.process() != journal.APPEND:
            continue
        for entry in reader:
            logger.debug("message keys %s" % ', '.join(entry.keys()))
            if 'MESSAGE' in entry:
                if entry.get('MESSAGE') != '':
                    search_client.index(index=args.index_name, body={
                        'priority': entry.get('PRIORITY'),
                        'syslog_facility': entry.get('SYSLOG_FACILITY'),
                        'machine_id': entry.get('_MACHINE_ID'),
                        'hostname': entry.get('_HOSTNAME'),
                        'message': entry.get('MESSAGE'),
                        'message_id': entry.get('MESSAGE_ID'),
                        'job_id': entry.get('JOB_ID'),
                        'job_type': entry.get('JOB_TYPE'),
                        'job_result': entry.get('JOB_RESULT'),
                        'timestamp': entry.get('__REALTIME_TIMESTAMP'),
                        'cmdline': entry.get('_CMDLINE'),
                        'boot_id': entry.get('_BOOT_ID'),
                        'pid': entry.get('_PID'),
                        'uid': entry.get('_UID'),
                        'gid': entry.get('_GID'),
                        'exe': entry.get('_EXE'),
                        'selinux_context': entry.get('_SELINUX_CONTEXT'),
                        'systemd_unit': entry.get('_SYSTEMD_UNIT'),
                        'systemd_slice': entry.get('_SYSTEMD_SLICE'),
                        'systemd_cgroup': entry.get('_SYSTEMD_CGROUP'),
                        'systemd_owner_uid': entry.get('_SYSTEMD_OWNER_UID'),
                        'systemd_session': entry.get('_SYSTEMD_SESSION')
                    })
                else:
                    logger.warning("empty message")
        # with open(args.config, "rb") as config_file:
        #     config = tomllib.load(config_file)
