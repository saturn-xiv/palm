import { useState, Fragment } from "react";
import PreviewOutlinedIcon from "@mui/icons-material/PreviewOutlined";
import IconButton from "@mui/material/IconButton";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import Button from "@mui/material/Button";
import { number as yup_number, object as yup_object } from "yup";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import { CopyToClipboard } from "react-copy-to-clipboard";

import {
  IAttachment,
  IAttachmentShowResponse,
  show_attachment_by_id,
} from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { useAppDispatch } from "../../../hooks";
import { error as error_box } from "../../../reducers/message-box";

interface IProps {
  item: IAttachment;
  small?: boolean;
}

const validationSchema = yup_object({
  days: yup_number().integer().required().min(1).max(7),
});

const Widget = ({ item, small }: IProps) => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const [open, setOpen] = useState<boolean>(false);
  const [attachment, setAttachment] = useState<
    IAttachmentShowResponse | undefined
  >();

  const formik = useFormik({
    enableReinitialize: true,
    initialValues: {
      days: 1,
    },
    validationSchema,
    onSubmit: (values) => {
      show_attachment_by_id(item.id, 60 * 60 * 24 * values.days)
        .then((res) => {
          setAttachment(res);
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
        <PreviewOutlinedIcon fontSize="inherit" />
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
        <DialogTitle id="alert-dialog-title">{attachment?.title}</DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
            {attachment && (
              <>
                <a href={attachment.url} target="_blank">
                  {attachment.title}
                </a>
                <CopyToClipboard text={attachment.url}>
                  <Button color="info" size="small">
                    <FormattedMessage id="buttons.copy-to-clipboard" />
                  </Button>
                </CopyToClipboard>
                <br />
                {attachment.contentType}
                &nbsp;
                {attachment.size}kb
              </>
            )}
          </DialogContentText>
          <TextField
            margin="normal"
            required
            fullWidth
            type="number"
            label={intl.formatMessage({ id: "form.fields.days.label" })}
            name="days"
            value={formik.values.days}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={formik.touched.days && Boolean(formik.errors.days)}
            helperText={formik.touched.days && formik.errors.days}
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
