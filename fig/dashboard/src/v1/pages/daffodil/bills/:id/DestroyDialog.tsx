import { useState, Fragment } from "react";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import { FormattedMessage, useIntl } from "react-intl";
import TextField from "@mui/material/TextField";
import DeleteOutlineOutlinedIcon from "@mui/icons-material/DeleteOutlineOutlined";
import { useNavigate } from "react-router-dom";

import { IBill, destroy_bill } from "../../../../api/daffodil";
import { useAppDispatch } from "../../../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../../../reducers/message-box";
import { IErrorMessage } from "../../../../api/graphql";

interface IProps {
  item: IBill;
}

const Widget = ({ item }: IProps) => {
  const [open, setOpen] = useState(false);
  const [reason, setReason] = useState("");
  const navigate = useNavigate();
  const intl = useIntl();
  const dispatch = useAppDispatch();

  return (
    <Fragment>
      <Button
        variant="contained"
        startIcon={<DeleteOutlineOutlinedIcon />}
        color="secondary"
        onClick={() => setOpen(true)}
      >
        <FormattedMessage id="buttons.delete" />
      </Button>
      <Dialog
        open={open}
        onClose={() => setOpen(false)}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description"
      >
        <DialogTitle id="alert-dialog-title">
          <FormattedMessage id="flashes.are-you-sure" />
        </DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
            <FormattedMessage
              id="daffodil.bills.destroy.confirm"
              values={{ id: item.id }}
            />
          </DialogContentText>
          <TextField
            margin="normal"
            required
            fullWidth
            label={intl.formatMessage({ id: "form.fields.reason.label" })}
            name="reason"
            multiline
            rows={4}
            value={reason}
            onChange={(e) => {
              if (e.target.value) {
                setReason(e.target.value);
              }
            }}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>
            <FormattedMessage id="buttons.cancel" />
          </Button>
          <Button
            onClick={() => {
              destroy_bill(item.id, reason)
                .then(() => {
                  dispatch(
                    success_box([intl.formatMessage({ id: "flashes.succeed" })])
                  );
                  navigate(`/dashboard/daffodil/ledgers/${item.ledger}`);
                })
                .catch((reason: IErrorMessage[]) => {
                  dispatch(error_box(reason.map((x) => x.message)));
                });
            }}
            autoFocus
          >
            <FormattedMessage id="buttons.confirm" />
          </Button>
        </DialogActions>
      </Dialog>
    </Fragment>
  );
};

export default Widget;
