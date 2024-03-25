import { useEffect, useState } from "react";
import Typography from "@mui/material/Typography";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";
import { FormattedMessage } from "react-intl";

import { logs as fetch_logs, IIndexLogResponse } from "../../api/camelia";
import Moment from "../../components/Moment";
import TablePagination from "../../components/TablePagination";

function Widget() {
  const [logs, setLogs] = useState<IIndexLogResponse>({
    items: [],
    pagination: {
      page: 1,
      size: 20,
      total: 0,
      hasNext: false,
      hasPrevious: false,
    },
  });
  useEffect(() => {
    fetch_logs(1, 20).then((res) => setLogs(res));
  }, []);
  return (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="users.logs.title" />
      </Typography>
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }} size="small">
          <TableHead>
            <TableRow>
              <TableCell align="center">
                <FormattedMessage id="form.fields.id.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.ip.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.message.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.created-at.label" />
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {logs.items.map((it, id) => (
              <TableRow hover key={id}>
                <TableCell align="center">{it.id}</TableCell>
                <TableCell>{it.ip}</TableCell>
                <TableCell>{it.message}</TableCell>
                <TableCell>
                  <Moment date={it.createdAt} />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
          <TablePagination
            size={logs.pagination.size}
            total={logs.pagination.total}
            page={logs.pagination.page}
            handleChange={(page, size) => {
              fetch_logs(page, size).then((res) => setLogs(res));
            }}
          />
        </Table>
      </TableContainer>
    </>
  );
}

export default Widget;
