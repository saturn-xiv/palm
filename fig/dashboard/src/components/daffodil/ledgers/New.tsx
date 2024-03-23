import { useState } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";
import { useNavigate } from "react-router-dom";

import { IErrorMessage } from "../../../api/graphql";
import { create_ledger } from "../../../api/daffodil";
import MessageBox, {
  IState as IMessageBox,
} from "../../../components/MessageBox";
import PictureSelect from "../../../components/attachments/PictureSelect";

const validationSchema = yup_object({
  name: yup_string().required().min(1).max(31),
  summary: yup_string().required().min(1).max(511),
});

function Widget() {
  const navigate = useNavigate();
  const [cover, setCover] = useState<number | null>(null);
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });
  const intl = useIntl();

  const formik = useFormik({
    initialValues: {
      name: "",
      summary: "",
    },
    validationSchema,
    onSubmit: (values) => {
      if (cover === null) {
        setMessageBox({
          messages: [
            intl.formatMessage({
              id: "attachments.select-picture.required",
            }),
          ],
          severity: "error",
        });
        return;
      }
      create_ledger(values.name, values.summary, cover)
        .then(() => {
          setMessageBox({
            messages: [intl.formatMessage({ id: "flashes.succeed" })],
            severity: "success",
          });
        })
        .catch((reason: IErrorMessage[]) => {
          setMessageBox({
            messages: reason.map((x) => x.message),
            severity: "error",
          });
        });
    },
  });
  return (
    <>
      <MessageBox
        severity={messageBox.severity}
        messages={messageBox.messages}
        handleClose={() => {
          setMessageBox({ messages: [], severity: "info" });
          navigate("/dashboard/daffodil/ledgers");
        }}
      />
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="daffodil.ledgers.new.title" />
      </Typography>
      <Box
        component="form"
        onSubmit={formik.handleSubmit}
        noValidate
        sx={{ mt: 1 }}
      >
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.name.label" })}
          name="name"
          value={formik.values.name}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.name && Boolean(formik.errors.name)}
          helperText={formik.touched.name && formik.errors.name}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.summary.label" })}
          name="summary"
          multiline
          rows={4}
          value={formik.values.summary}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.summary && Boolean(formik.errors.summary)}
          helperText={formik.touched.summary && formik.errors.summary}
        />
        <PictureSelect
          id={null}
          label={intl.formatMessage({ id: "form.fields.cover.label" })}
          form="daffodil.ledgers.new"
          handleChange={setCover}
        />
        <Button
          type="submit"
          fullWidth
          variant="contained"
          sx={{ mt: 3, mb: 2 }}
        >
          <FormattedMessage id="buttons.submit" />
        </Button>
      </Box>
    </>
  );
}

export default Widget;
