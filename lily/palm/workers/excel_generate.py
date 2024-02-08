import logging
import io

import pandas

from pumpkin import lily_pb2
from .. import save_s3_file


JOB = 'generate-excel'


def create_callback(s3):
    def it(ch, method, properties, body):
        logging.info("receive message %s", properties.message_id)
        _handle_message(body, s3)
        ch.basic_ack(delivery_tag=method.delivery_tag)
    return it


def _handle_message(message, s3):
    task = lily_pb2.ExcelGenerateTask()
    task.ParseFromString(message)
    logging.info("generate excel(%s, %s)", task.file.bucket, task.file.name)

    bio = io.BytesIO()
    writer = pandas.ExcelWriter(bio, engine="xlsxwriter")
    for name, sheet in task.excel.sheets:
        # https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.html
        df = pandas.DataFrame(data={'col1': [1, 2], 'col2': [3, 4]})
        df.to_excel(writer, scheet_name=name)
    writer.save()

    excel_size = bio.getbuffer().nbytes()
    bio.seek(0)
    excel_data = bio.read()
    save_s3_file(s3, task.file, excel_data, excel_size,
                 "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
