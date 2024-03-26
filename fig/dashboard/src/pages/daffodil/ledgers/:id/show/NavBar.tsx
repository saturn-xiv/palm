import { useState, Fragment } from "react";
import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import EditOutlinedIcon from "@mui/icons-material/EditOutlined";
import IosShareOutlinedIcon from "@mui/icons-material/IosShareOutlined";
import FileDownloadOutlinedIcon from "@mui/icons-material/FileDownloadOutlined";
import ReceiptLongOutlinedIcon from "@mui/icons-material/ReceiptLongOutlined";
import ArrowBackOutlinedIcon from "@mui/icons-material/ArrowBackOutlined";
import { FormattedMessage } from "react-intl";
import { useNavigate } from "react-router-dom";

import { ILedger } from "../../../../../api/daffodil";
import ExportDialog from "../ExportDialog";
import ShareDialog from "../ShareDialog";

interface IProps {
  item: ILedger;
}

const Widget = ({ item }: IProps) => {
  const [exportOpen, setExportOpen] = useState<boolean>(false);
  const [shareOpen, setShareOpen] = useState<boolean>(false);
  const navigate = useNavigate();
  return (
    <Stack spacing={2} direction="row">
      <Button
        onClick={() => navigate(`/dashboard/daffodil/ledgers/${item.id}/edit`)}
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
          navigate(`/dashboard/daffodil/ledgers/${item.id}/append-bill`)
        }
      >
        <FormattedMessage id="daffodil.ledgers.show.add-receipt" />
      </Button>
      <Fragment>
        <Button
          variant="contained"
          startIcon={<IosShareOutlinedIcon />}
          color="inherit"
          onClick={() => {
            setShareOpen(true);
          }}
        >
          <FormattedMessage id="buttons.share" />
        </Button>
        <ShareDialog
          item={item}
          open={shareOpen}
          handleClose={() => {
            setShareOpen(false);
          }}
        />
      </Fragment>
      <Fragment>
        <Button
          variant="contained"
          startIcon={<FileDownloadOutlinedIcon />}
          color="inherit"
          onClick={() => {
            setExportOpen(true);
          }}
        >
          <FormattedMessage id="buttons.export" />
        </Button>
        <ExportDialog
          item={item}
          open={exportOpen}
          handleClose={() => {
            setExportOpen(false);
          }}
        />
      </Fragment>
      <Button
        variant="contained"
        startIcon={<ArrowBackOutlinedIcon />}
        color="inherit"
        onClick={() => navigate("/dashboard/daffodil/ledgers")}
      >
        <FormattedMessage id="buttons.go-back" />
      </Button>
    </Stack>
  );
};

export default Widget;
