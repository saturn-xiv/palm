import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import { useParams, useNavigate } from "react-router-dom";
import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import EditOutlinedIcon from "@mui/icons-material/EditOutlined";
import IosShareOutlinedIcon from "@mui/icons-material/IosShareOutlined";
import FileDownloadOutlinedIcon from "@mui/icons-material/FileDownloadOutlined";
import ReceiptLongOutlinedIcon from "@mui/icons-material/ReceiptLongOutlined";
import { FormattedMessage } from "react-intl";

import { ILedger, show_ledger } from "../../../api/daffodil";

export function Component() {
  const navigate = useNavigate();
  const [item, setItem] = useState<ILedger | undefined>(undefined);
  const { id } = useParams();
  useEffect(() => {
    if (id) {
      show_ledger(parseInt(id, 10)).then((res) => {
        setItem(res);
      });
    }
  }, [id]);
  return item ? (
    <Grid item xs={12}>
      <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
        <Stack spacing={2} direction="row">
          <Button
            onClick={() =>
              navigate(`/dashboard/daffodil/ledgers/${item.id}/edit`)
            }
            variant="contained"
            startIcon={<EditOutlinedIcon />}
          >
            <FormattedMessage id="buttons.edit" />
          </Button>
          <Button
            variant="contained"
            startIcon={<ReceiptLongOutlinedIcon />}
            color="secondary"
            onClick={() =>
              navigate(`/dashboard/daffodil/ledgers/${item.id}/bills/new`)
            }
          >
            <FormattedMessage id="daffodil.ledgers.show.add-receipt" />
          </Button>
          <Button
            variant="contained"
            startIcon={<IosShareOutlinedIcon />}
            color="inherit"
          >
            <FormattedMessage id="buttons.share" />
          </Button>
          <Button
            variant="contained"
            startIcon={<FileDownloadOutlinedIcon />}
            color="inherit"
          >
            <FormattedMessage id="buttons.export" />
          </Button>
        </Stack>
      </Paper>
    </Grid>
  ) : (
    <></>
  );
}
