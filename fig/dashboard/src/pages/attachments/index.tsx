import { useEffect, useState } from "react";

import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";

import { index_attachment, IIndexAttachmentResponse } from "../../api/camelia";
import Upload from "./Upload";
import Table from "./PaginationTable";

export function Component() {
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
    <Grid item xs={12}>
      <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
        <Table
          items={attachments.items}
          pagination={attachments.pagination}
          handleRefresh={(page, size) => {
            index_attachment(page, size).then((res) => setAttachments(res));
          }}
        />
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
      </Paper>
    </Grid>
  );
}
