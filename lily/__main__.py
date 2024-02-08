import logging
import argparse
import sys
import tomllib


from palm import VERSION,   MinioClient, RabbitMqClient, is_stopped
from palm.tex import TEX_TO_PDF_JOB, TEX_TO_WORD_JOB, create_tex2pdf_queue_callback
from palm.excel import EXCEL_GENERATE_JOB, create_excel_generate_queue_callback
from palm.sms import SEND_SMS_JOB
from palm.email import SEND_EMAIL_JOB
from palm.server import Rpc as RpcServer

NAME = 'lily'


def start_rpc_server(args):
    rpc_server = RpcServer(args.port, args.threads)
    rpc_server.start()


def start_worker(args):
    logging.info('load configuration from %s', args.config.name)
    config = tomllib.load(args.config)
    rabbitmq_client = RabbitMqClient(config['rabbitmq'])
    logging.info('start %s consumer on %s' % (args.job, args.queue))

    if args.job == TEX_TO_PDF_JOB:
        minio_client = MinioClient(config['minio'])
        callback = create_tex2pdf_queue_callback(minio_client)
        rabbitmq_client.start_consuming(TEX_TO_PDF_JOB, callback)
    elif args.job == EXCEL_GENERATE_JOB:
        minio_client = MinioClient(config['minio'])
        callback = create_excel_generate_queue_callback(minio_client)
        rabbitmq_client.start_consuming(EXCEL_GENERATE_JOB, callback)
    else:
        logging.error('unimplemented job handler for %s', args.job)
        sys.exit(1)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        prog=NAME,
        description='A Tex/Epub/Excel/ImageMagick handler',
        epilog='https://github.com/saturn-xiv/palm')
    parser.add_argument('-d', '--debug',
                        action='store_true',
                        help='run on debug mode')

    parser.add_argument('-v', '--version',
                        action='store_true',
                        help=('print %s version' % NAME))

    subparsers = parser.add_subparsers(required=False)
    cmd_rpc = subparsers.add_parser(
        "rpc", help="start a gRPC server")
    cmd_rpc.add_argument("-p", "--port", type=int,
                         default=9999, help="listening port")
    cmd_rpc.add_argument("-t", "--threads", type=int,
                         default=1 << 5, help="max threads")
    cmd_rpc.set_defaults(func=start_rpc_server)

    cmd_worker = subparsers.add_parser(
        "worker", help="start a background queue consumer")
    cmd_worker.add_argument('-c', '--config',
                            type=argparse.FileType(mode='rb'),
                            default='config.toml',
                            help='load configuration(toml)')
    cmd_worker.add_argument("-j", "--job", required=True, type=str, choices=[
                            EXCEL_GENERATE_JOB, TEX_TO_PDF_JOB, TEX_TO_WORD_JOB, SEND_EMAIL_JOB, SEND_SMS_JOB], help="job name")
    cmd_worker.add_argument("-q", "--queue", required=True,
                            type=str, help="queue name")
    cmd_worker.set_defaults(func=start_worker)

    args = parser.parse_args()
    if args.version:
        print(VERSION)
        sys.exit()

    logging.basicConfig(level=(logging.DEBUG if args.debug else logging.INFO))
    if args.debug:
        logging.debug('run on debug mode')

    if is_stopped():
        logging.error('.stop file existed, quit...')
        sys.exit(1)

    args.func(args)
