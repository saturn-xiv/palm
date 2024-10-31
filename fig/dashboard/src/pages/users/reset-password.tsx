import { useState } from "react";
import Button from "@mui/material/Button";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import { useFormik } from "formik";
import Alert from "@mui/material/Alert";
import { useNavigate, useParams } from "react-router-dom";
import * as yup from "yup";

import Layout from "../../layouts/sign-in-side/Card";
import {
  IAlert,
  PASSWORD_MAX_LENGTH,
  PASSWORD_MIN_LENGTH,
  PASSWORD_PLACEHOLDER,
} from "../../components";
import { SIGN_IN_PATH } from "../../reducers/current-user";
import { reset_email_user_password_by_token } from "../../api/daffodil";
import { IError } from "../../api";
import NotFound from "../../components/NotFound";

export interface IFormValues {
  password: string;
  passwordConfirmation: string;
}

const validationSchema = yup.object({
  password: yup
    .string()
    .trim()
    .min(PASSWORD_MIN_LENGTH)
    .max(PASSWORD_MAX_LENGTH)
    .required(),
  passwordConfirmation: yup.string().oneOf([yup.ref("password")]),
});

const Widget = () => {
  const navigate = useNavigate();
  const initialValues: IFormValues = {
    password: "",
    passwordConfirmation: "",
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
  const { token } = useParams<{ token: string }>();

  const handleSubmit = (values: IFormValues) => {
    reset_email_user_password_by_token(token || "", values.password)
      .then(() => {
        setAlert({
          color: "success",
          messages: [
            intl.formatMessage({ id: "pages.users.reset-password.succeed" }),
          ],
        });
      })
      .catch((reason: IError[]) => {
        setAlert({
          color: "error",
          messages: reason.map((x) => x.message),
        });
      });
  };

  return token ? (
    <Layout
      title={intl.formatMessage({ id: "pages.users.reset-password.title" })}
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
        <FormLabel htmlFor="password">
          <FormattedMessage id="form.fields.password.label" />
        </FormLabel>
        <TextField
          value={formik.values.password}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.password && Boolean(formik.errors.password)}
          helperText={formik.touched.password && formik.errors.password}
          color={formik.errors.password ? "error" : "primary"}
          name="password"
          placeholder={PASSWORD_PLACEHOLDER}
          type="password"
          autoFocus
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "password" }}
        />
      </FormControl>
      <FormControl>
        <FormLabel htmlFor="passwordConfirmation">
          <FormattedMessage id="form.fields.password-confirmation.label" />
        </FormLabel>
        <TextField
          value={formik.values.passwordConfirmation}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={
            formik.touched.passwordConfirmation &&
            Boolean(formik.errors.passwordConfirmation)
          }
          helperText={
            formik.touched.passwordConfirmation &&
            formik.errors.passwordConfirmation
          }
          color={formik.errors.passwordConfirmation ? "error" : "primary"}
          name="passwordConfirmation"
          placeholder={PASSWORD_PLACEHOLDER}
          type="password"
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "passwordConfirmation" }}
        />
      </FormControl>
      <Button type="submit" fullWidth variant="contained">
        <FormattedMessage id="buttons.submit" />
      </Button>
    </Layout>
  ) : (
    <NotFound />
  );
};

export default Widget;
