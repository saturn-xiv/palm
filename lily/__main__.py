import logging
import argparse
import sys
import tomllib
import multiprocessing


from palm import VERSION, TwilioClient, MinioClient, RabbitMqClient, SmtpClient, is_stopped
from palm.workers.tex_to_pdf import JOB as TEX_TO_PDF_JOB, create_callback as create_tex2pdf_queue_callback
from palm.workers.excel_generate import JOB as EXCEL_GENERATE_JOB, create_callback as create_excel_generate_queue_callback
from palm.workers.sms_send import JOB as SEND_SMS_JOB, create_callback as create_sms_send_queue_callback
from palm.workers.email_send import JOB as SEND_EMAIL_JOB, create_callback as create_email_send_queue_callback
from palm.workers.tex_to_word import JOB as TEX_TO_WORD_JOB
from palm.workers.picture_convert import JOB as CONVERT_PICTURE_JOB
from palm.rpc import Server as RpcServer
from palm.controllers import launch_server as launch_http_server

NAME = 'lily'

_NUMBER_OF_WORKERS = (multiprocessing.cpu_count() * 2) + 1


def start_web_server(args):
    launch_http_server(args.port, args.threads)


def start_rpc_server(args):
    server = RpcServer(args.port, args.threads)
    server.start()


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
    elif args.job == SEND_EMAIL_JOB:
        smtp_client = SmtpClient(config['smtp'])
        callback = create_email_send_queue_callback(smtp_client)
        rabbitmq_client.start_consuming(SEND_EMAIL_JOB, callback)
    elif args.job == SEND_SMS_JOB:
        twilio_client = TwilioClient(config['twilio'])
        callback = create_sms_send_queue_callback(twilio_client)
        rabbitmq_client.start_consuming(SEND_SMS_JOB, callback)
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

    web_rpc = subparsers.add_parser(
        "web", help="start a HTTP server")
    web_rpc.add_argument("-p", "--port", type=int,
                         default=8080, help="listening port")
    web_rpc.add_argument("-t", "--threads", type=int,
                         default=_NUMBER_OF_WORKERS, help="number of workers")
    web_rpc.set_defaults(func=start_web_server)

    cmd_rpc = subparsers.add_parser(
        "rpc", help="start a gRPC server")
    cmd_rpc.add_argument("-p", "--port", type=int,
                         default=9999, help="listening port")
    cmd_rpc.add_argument("-t", "--threads", type=int,
                         default=_NUMBER_OF_WORKERS, help="number of workers")
    cmd_rpc.set_defaults(func=start_rpc_server)

    cmd_worker = subparsers.add_parser(
        "worker", help="start a background queue consumer")
    cmd_worker.add_argument('-c', '--config',
                            type=argparse.FileType(mode='rb'),
                            default='config.toml',
                            help='load configuration(toml)')
    cmd_worker.add_argument("-j", "--job", required=True, type=str, choices=[
                            EXCEL_GENERATE_JOB, TEX_TO_PDF_JOB, TEX_TO_WORD_JOB, CONVERT_PICTURE_JOB, SEND_EMAIL_JOB, SEND_SMS_JOB], help="job name")
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
