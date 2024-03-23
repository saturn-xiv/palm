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

import { index_attachment, IIndexAttachmentResponse } from "../../api/camelia";
import { timestamp_to_str } from "../../utils";
import TablePagination from "../../components/TablePagination";
import Upload from "./Upload";

function Widget() {
  const [attachments, setAttachments] = useState<IIndexAttachmentResponse>({
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
    index_attachment(1, 20).then((res) => setAttachments(res));
  }, []);
  return (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="attachments.index.title" />
      </Typography>
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }} size="small">
          <TableHead>
            <TableRow>
              <TableCell align="center">
                <FormattedMessage id="form.fields.id.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.title.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.content-type.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.updated-at.label" />
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {attachments.items.map((it, id) => (
              <TableRow hover key={id}>
                <TableCell align="center">{it.id}</TableCell>
                <TableCell>{it.title}</TableCell>
                <TableCell>{it.contentType}</TableCell>
                <TableCell>{timestamp_to_str(it.updatedAt)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
          <TablePagination
            size={attachments.pagination.size}
            total={attachments.pagination.total}
            page={attachments.pagination.page}
            handleChange={(page, size) => {
              index_attachment(page, size).then((res) => setAttachments(res));
            }}
          />
        </Table>
      </TableContainer>
      <br />
      <Upload
        accept={{
          "image/png": [".png"],
          "image/svg+xml": [".svg"],
          "image/jpeg": [".jpg", ".jpeg"],
          "audio/mpeg": [".mp3"],
          "video/mp4": [".mp4"],
          "application/pdf": [".pdf"],
          "text/plain": [".txt"],
          "application/x-tar": [".tar"],
          "application/zip": [".zip"],
          "application/x-7z-compressed": [".7z"],
          "application/vnd.rar": [".rar"],
        }}
        handleRefresh={() => {
          index_attachment(1, 20).then((res) => setAttachments(res));
        }}
      />
    </>
  );
}

export default Widget;
