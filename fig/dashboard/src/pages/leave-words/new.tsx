import { useState } from "react";
import Button from "@mui/material/Button";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import { useFormik } from "formik";
import Alert from "@mui/material/Alert";
import { useNavigate } from "react-router-dom";
import * as yup from "yup";

import Layout from "../../layouts/sign-in-side/Card";
import { IAlert } from "../../components";
import { SIGN_IN_PATH } from "../../reducers/current-user";
import { create_leave_word } from "../../api/daffodil";
import { IError } from "../../api";

export interface IFormValues {
  content: string;
}
const validationSchema = yup.object({
  content: yup.string().trim().min(31).max(5119).required(),
});

const Widget = () => {
  const navigate = useNavigate();
  const initialValues: IFormValues = {
    content: "",
  };
  const formik = useFormik({
    initialValues,
    validationSchema,
    onSubmit: (values) => {
      handleSubmit(values);
    },
  });
  const [alert, setAlert] = useState<IAlert>();
  const intl = useIntl();

  const handleSubmit = (values: IFormValues) => {
    create_leave_word(values.content)
      .then(() => {
        setAlert({
          color: "success",
          messages: [intl.formatMessage({ id: "flashes.succeed" })],
        });
      })
      .catch((reason: IError[]) => {
        setAlert({
          color: "error",
          messages: reason.map((x) => x.message),
        });
      });
  };

  return (
    <Layout
      title={intl.formatMessage({ id: "pages.leave-words.new.title" })}
      handleSubmit={formik.handleSubmit}
    >
      {alert && (
        <Alert
          severity={alert.color}
          onClose={() => {
            if (alert.color == "success") {
              navigate(SIGN_IN_PATH);
            }
          }}
        >
          {alert.messages.map((x, i) => (
            <div key={i}>{x}</div>
          ))}
        </Alert>
      )}
      <FormControl>
        <FormLabel htmlFor="content">
          <FormattedMessage id="form.fields.content.label" />
        </FormLabel>
        <TextField
          value={formik.values.content}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.content && Boolean(formik.errors.content)}
          helperText={formik.touched.content && formik.errors.content}
          color={formik.errors.content ? "error" : "primary"}
          name="content"
          multiline
          minRows={4}
          autoFocus
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "content" }}
        />
      </FormControl>
      <Button type="submit" fullWidth variant="contained">
        <FormattedMessage id="buttons.submit" />
      </Button>
    </Layout>
  );
};

export default Widget;
