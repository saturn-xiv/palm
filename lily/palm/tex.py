import logging
import tempfile
import os.path
import subprocess


from io import BytesIO


from pumpkin import lily_pb2
from . import save_s3_file

TEX_TO_PDF_JOB = 'tex-to-pdf'
TEX_TO_WORD_JOB = 'tex-to-word'


def create_tex2pdf_queue_callback(s3):
    def it(ch, method, properties, body):
        logging.info("receive message %s", properties.message_id)
        _handle_tex2pdf_message(body, s3)
        ch.basic_ack(delivery_tag=method.delivery_tag)
    return it


def _handle_tex2pdf_message(message, s3):
    task = lily_pb2.TexTask()
    task.ParseFromString(message)
    logging.info("convert tex to pdf(%d) %s ", len(task.files), task.title)
    with tempfile.TemporaryDirectory(prefix='tex-') as root:
        for name in task.files:
            with open(os.path.join(root, name), mode='wb') as fd:
                logging.debug("generate file %s/%s", root, name)
                fd.write(task.files[name])
        for _ in range(2):
            try:
                subprocess.run(
                    ['xelatex', '-halt-on-error', 'main.tex'],  check=True, cwd=root)
            except subprocess.CalledProcessError as e:
                logging.error("%s", e)
                return

        pdf_file = os.path.join(root, 'main.pdf')
        pdf_size = os.path.getsize(pdf_file)
        with open(pdf_file, mode="rb") as fd:
            pdf_data = BytesIO(fd.read())
            save_s3_file(s3, task.file, pdf_data, pdf_size, "application/pdf")
