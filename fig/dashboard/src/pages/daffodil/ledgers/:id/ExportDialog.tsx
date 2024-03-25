import Button from "@mui/material/Button";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import { FormattedMessage, useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";
import { useFormik } from "formik";

import { ILedger, export_ledger } from "../../../../api/daffodil";
import { useAppDispatch } from "../../../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../../../reducers/message-box";
import { IErrorMessage } from "../../../../api/graphql";

const validationSchema = yup_object({
  format: yup_string().required(),
});

interface IProps {
  item: ILedger;
  open: boolean;
  handleClose: () => void;
}

const Widget = ({ item, open, handleClose }: IProps) => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const formik = useFormik({
    enableReinitialize: true,
    initialValues: {
      format: "HTML",
    },
    validationSchema,
    onSubmit: (values) => {
      export_ledger(item.id, values.format)
        .then(() => {
          dispatch(
            success_box([
              intl.formatMessage({ id: "daffodil.ledgers.export.succeed" }),
            ])
          );
        })
        .catch((reason: IErrorMessage[]) => {
          dispatch(error_box(reason.map((x) => x.message)));
        });
    },
  });
  return (
    <Dialog
      open={open}
      onClose={handleClose}
      PaperProps={{
        component: "form",
        onSubmit: formik.handleSubmit,
      }}
    >
      <DialogTitle>
        <FormattedMessage
          id="daffodil.ledgers.export.title"
          values={{ name: item.name }}
        />
      </DialogTitle>
      <DialogContent>
        <DialogContentText>
          <FormattedMessage id="daffodil.ledgers.export.summary" />
        </DialogContentText>
        <FormControl margin="normal" required fullWidth>
          <InputLabel id="daffodil-ledger-export-format-select-label">
            <FormattedMessage id="form.fields.format.label" />
          </InputLabel>
          <Select
            labelId="daffodil-ledger-export-type-select-label"
            name="format"
            value={formik.values.format}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          >
            {["PDF", "HTML"].map((x, i) => (
              <MenuItem key={i} value={x}>
                {x}
              </MenuItem>
            ))}
          </Select>
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
