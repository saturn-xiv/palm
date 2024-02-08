import logging
import argparse
import sys
import tomllib


from palm import VERSION,   MinioClient, RabbitMqClient, is_stopped
from palm.tex import TEX2PDF_QUEUE, create_tex2pdf_queue_callback
from palm.server import Rpc as RpcServer
from gourd.v1.constants import SEND_EMAIL_QUEUE_NAME, SEND_SMS_QUEUE_NAME, TEX_TO_PDF_QUEUE_NAME, TEX_TO_WORD_QUEUE_NAME

NAME = 'lily'


def _queue_name(n):
    return "palm.lily.%s.v%s" % (n, VERSION)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        prog=NAME,
        description='A Tex/Epub/Excel/ImageMagick handler',
        epilog='https://github.com/saturn-xiv/palm')
    parser.add_argument('-c', '--config',
                        type=argparse.FileType(mode='rb'),
                        default='config.toml',
                        help='load configuration(toml)')
    parser.add_argument('-d', '--debug',
                        action='store_true',
                        help='run on debug mode')
    parser.add_argument('-r', '--rpc',
                        action='store_true',
                        help='run a rpc server')
    parser.add_argument('-w', '--worker',
                        help='run a queue worker: [%s, %s, %s, %s]' % (TEX_TO_WORD_QUEUE_NAME, TEX_TO_PDF_QUEUE_NAME, SEND_EMAIL_QUEUE_NAME, SEND_SMS_QUEUE_NAME))
    parser.add_argument('-v', '--version',
                        action='store_true',
                        help=('print %s version' % NAME))
    args = parser.parse_args()
    if args.version:
        print(VERSION)
        sys.exit()
    logging.basicConfig(level=(logging.DEBUG if args.debug else logging.INFO))
    if args.debug:
        logging.debug('run on debug mode with %s', args)

    if is_stopped():
        logging.error('.stop file existed, quit...')
        sys.exit(1)

    logging.info('load configuration from %s', args.config.name)

    config = tomllib.load(args.config)
    minio_client = MinioClient(config['minio'])
    rabbitmq_client = RabbitMqClient(config['rabbitmq'])
    if args.worker:
        if args.worker == TEX_TO_PDF_QUEUE_NAME:
            callback = create_tex2pdf_queue_callback(minio_client)
            rabbitmq_client.start_consuming(TEX_TO_PDF_QUEUE_NAME, callback)
        else:
            logging.error('unimplemented queue %s', args.worker)
            sys.exit(1)
        sys.exit()
    rpc_server = RpcServer(
        config['rpc'], minio_client, rabbitmq_client)
    rpc_server.start()
