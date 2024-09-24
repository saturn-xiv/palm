# -*- coding: utf-8 -*-

import logging
import argparse

from systemd_journal import launch as launch_systemd_journal
from snmp_fetch import launch as launch_snmp_fetch


logger = logging.getLogger(__name__)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='azalea',
        description='A logger/snmp crawler',
        epilog='https://github.com/saturn-xiv/palm')
    parser.add_argument('-c', '--config', default="config.toml",
                        help="load configuration")
    parser.add_argument(
        '-d', '--debug', help="run on debug mode", action='store_true')
    parser.add_argument('-v', '--verbose',
                        help="print version", action='version', version='2024.9.18')

    subparsers = parser.add_subparsers(required=True, help='sub-commands help')

    cmd_systemd_journal = subparsers.add_parser(
        'systemd-journal', help='systemd journal crawler')
    cmd_systemd_journal.add_argument(
        '-n', '--index-name', required=True, help='OpenSearch index name')

    cmd_systemd_journal.set_defaults(func=launch_systemd_journal)

    cmd_snmp_fetch = subparsers.add_parser(
        'snmp-fetch', help='fetch monitor data from snmp')
    cmd_snmp_fetch.add_argument(
        '-H', '--host', required=True, help='hostname')
    cmd_snmp_fetch.add_argument(
        '-p', '--port', default=161, type=int, help='port')

    cmd_snmp_fetch.set_defaults(func=launch_snmp_fetch)

    args = parser.parse_args()
    logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',
                        level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("run on debug mode")
    logger.info("load configuration from %s" % args.config)
    args.func(args)
