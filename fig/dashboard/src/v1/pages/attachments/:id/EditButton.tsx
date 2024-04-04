import { useState, Fragment } from "react";
import IconButton from "@mui/material/IconButton";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import Button from "@mui/material/Button";
import { string as yup_string, object as yup_object } from "yup";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import EditOutlinedIcon from "@mui/icons-material/EditOutlined";
import { FormattedMessage, useIntl } from "react-intl";

import { IAttachment, update_attachment } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { useAppDispatch } from "../../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../../reducers/message-box";

interface IProps {
  item: IAttachment;
  small?: boolean;
  handleRefresh: () => void;
}

const validationSchema = yup_object({
  title: yup_string().required().min(1).max(127),
});

const Widget = ({ item, small, handleRefresh }: IProps) => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const [open, setOpen] = useState<boolean>(false);

  const formik = useFormik({
    enableReinitialize: true,
    initialValues: {
      title: item.title,
    },
    validationSchema,
    onSubmit: (values) => {
      update_attachment(item.id, values.title)
        .then(() => {
          dispatch(
            success_box([intl.formatMessage({ id: "flashes.succeed" })])
          );
          handleRefresh();
        })
        .catch((reason: IErrorMessage[]) => {
          dispatch(error_box(reason.map((x) => x.message)));
        });
    },
  });
  return (
    <Fragment>
      <IconButton
        onClick={() => {
          setOpen(true);
        }}
        aria-label="preview"
        size={small ? "small" : "medium"}
      >
        <EditOutlinedIcon fontSize="inherit" />
      </IconButton>

      <Dialog
        open={open}
        onClose={() => {
          setOpen(false);
        }}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description"
        PaperProps={{
          component: "form",
          onSubmit: formik.handleSubmit,
        }}
      >
        <DialogTitle id="alert-dialog-title">{item.id}</DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
            {item.contentType}
            &nbsp;
            {item.size}kb
          </DialogContentText>
          <TextField
            margin="normal"
            required
            fullWidth
            label={intl.formatMessage({ id: "form.fields.title.label" })}
            name="title"
            value={formik.values.title}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={formik.touched.title && Boolean(formik.errors.title)}
            helperText={formik.touched.title && formik.errors.title}
          />
        </DialogContent>
        <DialogActions>
          <Button
            variant="contained"
            color="inherit"
            onClick={() => {
              setOpen(false);
            }}
          >
            <FormattedMessage id="buttons.close" />
          </Button>
          <Button variant="contained" type="submit">
            <FormattedMessage id="buttons.submit" />
          </Button>
        </DialogActions>
      </Dialog>
    </Fragment>
  );
};

export default Widget;
