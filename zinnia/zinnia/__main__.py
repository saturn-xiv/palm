import sys
import logging
import argparse
import multiprocessing
import importlib.metadata

from . import web

logger = logging.getLogger(__name__)


def launch_web_server(args):
    if args.workers < 4:
        logger.error("num of workers must too small(at last 4)")
        return
    addr = '%s:%s' % ('127.0.0.1', args.port)
    logger.info("start a http server listening on http://%s with %d workers",
                addr, args.workers)

    web.StandaloneApplication(
        web.create_app(args.debug, args.config),
        {
            'bind': addr,
            'workers': args.workers,
            'worker_class': 'gevent',
        }
    ).run()


def launch_rpc_server(args):
    logger.info("start a gRPC server listening on tcp://127.0.0.1:%d", args.port)
    # TODO


def launch_queue_consumer(args):
    logger.info("start a queue consumer(%s) for job %s with %ds task intervals",
                args.name, args.job, args.interval)
    # TODO


def list_user(args):
    pass


def create_user_by_email(args):
    logger.info("create an email user %s<%s>", args.name, args.email)
    # TODO


def add_roles_for_user(args):
    logger.info("add roles(%s) for user(%s)", ",".join(args.roles), args.user)
    # TODO


def delete_roles_for_user(args):
    logger.info("delete roles(%s) for user(%s)",
                ",".join(args.roles), args.user)
    # TODO


def main():
    parser = argparse.ArgumentParser(
        description='A total free education & translation solution.',
        epilog='https://github.com/saturn-xiv/palm')
    parser.add_argument('-d', '--debug', action='store_true',
                        help='run on debug mode')
    parser.add_argument(
        '-c', '--config', help='configuration file', default='config.toml')
    parser.add_argument('-v', '--version', action='version',
                        version='%(prog)s ({version})'.format(version=importlib.metadata.version('zinnia')))
    subparsers = parser.add_subparsers(required=True, help='sub-commands help')

    parser_web = subparsers.add_parser('web', help='start a http server')
    parser_web.add_argument('-p', '--port', type=int, default=8080)
    parser_web.add_argument('-w', '--workers', type=int,
                            default=(multiprocessing.cpu_count() * 2) + 1)
    parser_web.set_defaults(func=launch_web_server)

    parser_rpc = subparsers.add_parser('rpc', help='start a gRPC server')
    parser_rpc.add_argument('-p', '--port', type=int, default=8080)
    parser_rpc.set_defaults(func=launch_rpc_server)

    parser_consumer = subparsers.add_parser(
        'consumer', help='start a queue consumer')
    parser_consumer.add_argument('-i', '--interval', type=int,
                                 default=1, help='intervals between tasks(in seconds)')
    parser_consumer.add_argument('-n', '--name', required=True)
    parser_consumer.add_argument(
        '-j', '--job', choices=('sms-send', 'email-send', 'tex-to-pdf'), required=True)
    parser_consumer.set_defaults(func=launch_queue_consumer)

    parser_create_user = subparsers.add_parser(
        'create-user', help='create user by email')
    parser_create_user.add_argument('-n', '--name', required=True)
    parser_create_user.add_argument('-e', '--email', required=True)
    parser_create_user.add_argument('-p', '--password', required=True)
    parser_create_user.set_defaults(func=create_user_by_email)

    parser_list_user = subparsers.add_parser(
        'list-user', help='list all users')
    parser_list_user.set_defaults(func=list_user)

    parser_add_roles_for_user = subparsers.add_parser(
        'add-roles-for-user', help='add roles for user')
    parser_add_roles_for_user.add_argument(
        '-u', '--user', required=True, help="user's uid")
    parser_add_roles_for_user.add_argument(
        '-r', '--roles', required=True, action='extend', nargs="+", type=str)
    parser_add_roles_for_user.set_defaults(func=add_roles_for_user)

    parser_delete_roles_for_user = subparsers.add_parser(
        'delete-roles-for-user', help='add roles for user')
    parser_delete_roles_for_user.add_argument(
        '-u', '--user', required=True, help="user's uid")
    parser_delete_roles_for_user.add_argument(
        '-r', '--roles', required=True, action='extend', nargs="+", type=str)
    parser_delete_roles_for_user.set_defaults(func=delete_roles_for_user)

    args = parser.parse_args()
    logging.basicConfig(level=logging.DEBUG if args.debug else logging.INFO)
    logger.debug("run on debug mode")

    args.func(args)


if __name__ == "__main__":
    main()
