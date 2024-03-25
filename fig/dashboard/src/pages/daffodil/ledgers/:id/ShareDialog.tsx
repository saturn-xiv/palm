import { useState, FormEvent } from "react";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import { DatePicker } from "@mui/x-date-pickers/DatePicker";
import FormControl from "@mui/material/FormControl";
import Alert from "@mui/material/Alert";
import CheckIcon from "@mui/icons-material/Check";
import Typography from "@mui/material/Typography";
import { FormattedMessage, useIntl } from "react-intl";
import moment, { Moment } from "moment";
import { CopyToClipboard } from "react-copy-to-clipboard";

import { ILedger, share_ledger } from "../../../../api/daffodil";
import { DATE_PICKER_FORMAT } from "../../../../components";
import { useAppDispatch } from "../../../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../../../reducers/message-box";
import { IErrorMessage } from "../../../../api/graphql";

interface IProps {
  item: ILedger;
  open: boolean;
  handleClose: () => void;
}

const Widget = ({ item, open, handleClose }: IProps) => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const [begin, setBegin] = useState<Moment>(moment());
  const [end, setEnd] = useState<Moment>(moment().add(1, "months"));
  const [url, setUrl] = useState<string | null>(null);

  return (
    <Dialog
      open={open}
      onClose={handleClose}
      PaperProps={{
        component: "form",
        onSubmit: (event: FormEvent<HTMLFormElement>) => {
          event.preventDefault();
          share_ledger(
            item.id,
            begin.format(DATE_PICKER_FORMAT),
            end.format(DATE_PICKER_FORMAT)
          )
            .then((res: string) => {
              dispatch(
                success_box([intl.formatMessage({ id: "flashes.succeed" })])
              );
              setUrl(new URL(res, window.location.href).toString());
            })
            .catch((reason: IErrorMessage[]) => {
              dispatch(error_box(reason.map((x) => x.message)));
            });
        },
      }}
    >
      <DialogTitle>
        <FormattedMessage
          id="daffodil.ledgers.share.title"
          values={{ name: item.name }}
        />
      </DialogTitle>
      <DialogContent>
        <DialogContentText>
          <FormattedMessage id="daffodil.ledgers.share.summary" />
        </DialogContentText>
        <FormControl margin="normal" required>
          <DatePicker<Moment>
            label={<FormattedMessage id="form.fields.from.label" />}
            name="begin"
            value={begin}
            format={DATE_PICKER_FORMAT}
            onChange={(v) => {
              if (v) {
                setBegin(v);
              }
            }}
          />
        </FormControl>
        &nbsp;
        <FormControl margin="normal" required>
          <DatePicker<Moment>
            label={<FormattedMessage id="form.fields.to.label" />}
            name="end"
            value={end}
            onChange={(v) => {
              if (v) {
                setEnd(v);
              }
            }}
            format={DATE_PICKER_FORMAT}
          />
        </FormControl>
        {url && (
          <Alert
            icon={<CheckIcon fontSize="inherit" />}
            severity="success"
            action={
              <CopyToClipboard text={url}>
                <Button color="inherit" size="small">
                  <FormattedMessage id="buttons.copy" />
                </Button>
              </CopyToClipboard>
            }
          >
            <Typography noWrap>{url}</Typography>
          </Alert>
        )}
      </DialogContent>
      <DialogActions>
        <Button onClick={handleClose}>
          <FormattedMessage id="buttons.cancel" />
        </Button>
        <Button type="submit">
          <FormattedMessage id="buttons.submit" />
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default Widget;
