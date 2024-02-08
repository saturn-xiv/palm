import logging
import io

import pandas

from pumpkin import lily_pb2, lily_pb2_grpc
from . import save_s3_file


EXCEL_GENERATE_JOB = 'generate-excel'

class Service(lily_pb2_grpc.ExcelServicer):
    def Parse(self, request, context):
        logging.info("parse excel(%d)", len(request.payload))
        response = lily_pb2.ExcelParseResponse()

        file = io.BytesIO(request.payload)
        doc = pandas.read_excel(file, sheet_name=None)
        for name, sheet in doc:
            logging.info("find sheet %s", name)
            sht = response.sheets.add()
            sht.name = name
            for row, item in sheet.to_dict().items():
                for col, val in item.items():
                    logging.debug('find (%s, %s, %s)', row, col, val)
                    cell = sht.cells.add()
                    cell.row = row
                    cell.col = col
                    cell.val = val
        return response


def create_excel_generate_queue_callback(s3):
    def it(ch, method, properties, body):
        logging.info("receive message %s", properties.message_id)
        _handle_excel_generate_message(body, s3)
        ch.basic_ack(delivery_tag=method.delivery_tag)
    return it


def _handle_excel_generate_message(message, s3):
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
