import { useState, FormEvent } from "react";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import { DatePicker } from "@mui/x-date-pickers/DatePicker";
import FormControl from "@mui/material/FormControl";
import { FormattedMessage } from "react-intl";
import moment, { Moment } from "moment";

import { ILedger } from "../../../../api/daffodil";
import { DATE_PICKER_FORMAT } from "../../../../components";

interface IProps {
  item: ILedger;
  open: boolean;
  handleClose: () => void;
}

const Widget = ({ item, open, handleClose }: IProps) => {
  const [begin, setBegin] = useState<Moment>(moment());
  const [end, setEnd] = useState<Moment>(moment().add(1, "months"));

  return (
    <Dialog
      open={open}
      onClose={handleClose}
      PaperProps={{
        component: "form",
        onSubmit: (event: FormEvent<HTMLFormElement>) => {
          event.preventDefault();
          console.log(
            begin.format(DATE_PICKER_FORMAT),
            end.format(DATE_PICKER_FORMAT)
          );
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
