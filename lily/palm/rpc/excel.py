import logging
import io

import pandas

from pumpkin import lily_pb2, lily_pb2_grpc


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
