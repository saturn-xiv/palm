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
import {
  EMAIL_MAX_LENGTH,
  EMAIL_MIN_LENGTH,
  IAlert,
  NAME_MAX_LENGTH,
  NAME_MIN_LENGTH,
  PASSWORD_MAX_LENGTH,
  PASSWORD_MIN_LENGTH,
  PASSWORD_PLACEHOLDER,
} from "../../components";
import { SIGN_IN_PATH } from "../../reducers/current-user";
import { user_sign_up_by_email } from "../../api/daffodil";
import { guess_timezone } from "../../utils";
import { IError } from "../../api";

interface IEmailFormProps {
  title: string;
  handleSubmit: (values: IEmailFormValues) => void;
  alert?: IAlert;
}

export interface IEmailFormValues {
  email: string;
  realName: string;
  nickname: string;
  password: string;
  passwordConfirmation: string;
}

const validationSchema = yup.object({
  email: yup
    .string()
    .trim()
    .min(EMAIL_MIN_LENGTH)
    .max(EMAIL_MAX_LENGTH)
    .email()
    .required(),
  password: yup
    .string()
    .trim()
    .min(PASSWORD_MIN_LENGTH)
    .max(PASSWORD_MAX_LENGTH)
    .required(),
  nickname: yup
    .string()
    .trim()
    .min(NAME_MIN_LENGTH)
    .max(NAME_MAX_LENGTH)
    .required(),
  realName: yup
    .string()
    .trim()
    .min(NAME_MIN_LENGTH)
    .max(NAME_MAX_LENGTH)
    .required(),
  passwordConfirmation: yup.string().oneOf([yup.ref("password")]),
});

export const EmailForm = ({ title, alert, handleSubmit }: IEmailFormProps) => {
  const navigate = useNavigate();
  const initialValues: IEmailFormValues = {
    email: "",
    realName: "",
    nickname: "",
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

  return (
    <Layout title={title} handleSubmit={formik.handleSubmit}>
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
        <FormLabel htmlFor="realName">
          <FormattedMessage id="form.fields.real-name.label" />
        </FormLabel>
        <TextField
          value={formik.values.realName}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.realName && Boolean(formik.errors.realName)}
          helperText={formik.touched.realName && formik.errors.realName}
          color={formik.errors.realName ? "error" : "primary"}
          type="text"
          name="realName"
          autoFocus
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "realName" }}
        />
      </FormControl>
      <FormControl>
        <FormLabel htmlFor="email">
          <FormattedMessage id="form.fields.email.label" />
        </FormLabel>
        <TextField
          value={formik.values.email}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.email && Boolean(formik.errors.email)}
          helperText={formik.touched.email && formik.errors.email}
          color={formik.errors.email ? "error" : "primary"}
          type="email"
          name="email"
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "email" }}
        />
      </FormControl>
      <FormControl>
        <FormLabel htmlFor="nickname">
          <FormattedMessage id="form.fields.nickname.label" />
        </FormLabel>
        <TextField
          value={formik.values.nickname}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.nickname && Boolean(formik.errors.nickname)}
          helperText={formik.touched.nickname && formik.errors.nickname}
          color={formik.errors.nickname ? "error" : "primary"}
          type="text"
          name="nickname"
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "nickname" }}
        />
      </FormControl>
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
  );
};

const Widget = () => {
  const [alert, setAlert] = useState<IAlert>();
  const intl = useIntl();
  const handleSubmit = (values: IEmailFormValues) => {
    user_sign_up_by_email({
      realName: values.realName,
      nickname: values.nickname,
      email: values.email,
      password: values.password,
      timezone: guess_timezone(),
    })
      .then(() => {
        setAlert({
          color: "success",
          messages: [
            intl.formatMessage({ id: "pages.users.sign-up.instruction" }),
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
  return (
    <EmailForm
      title={intl.formatMessage({ id: "pages.users.sign-up.title" })}
      alert={alert}
      handleSubmit={handleSubmit}
    />
  );
};
export default Widget;
