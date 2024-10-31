import { useState } from "react";
import Button from "@mui/material/Button";
import Checkbox from "@mui/material/Checkbox";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import FormControlLabel from "@mui/material/FormControlLabel";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import { useFormik } from "formik";
import Alert from "@mui/material/Alert";
import { useNavigate } from "react-router-dom";
import * as yup from "yup";

import Layout from "../../layouts/sign-in-side/Card";
import {
  IAlert,
  PASSWORD_MIN_LENGTH,
  PASSWORD_PLACEHOLDER,
} from "../../components";
import { PERSONAL_PATH } from "../../reducers/current-user";
import { user_sign_in_by_email } from "../../api/daffodil";
import { IError } from "../../api";
import { useAppDispatch } from "../../hooks";
import { signIn } from "../../reducers/current-user";

export interface IFormValues {
  user: string;
  password: string;
  rememberMe: boolean;
}

const validationSchema = yup.object({
  user: yup.string().trim().required(),
  password: yup.string().trim().min(PASSWORD_MIN_LENGTH).required(),
});

const Widget = () => {
  const [alert, setAlert] = useState<IAlert>();
  const intl = useIntl();
  const navigate = useNavigate();
  const dispatch = useAppDispatch();
  const initialValues: IFormValues = {
    user: "",
    password: "",
    rememberMe: true,
  };
  const formik = useFormik({
    initialValues,
    validationSchema,
    onSubmit: (values) => {
      user_sign_in_by_email(values.user, values.password)
        .then((res) => {
          setAlert({
            color: "success",
            messages: [
              intl.formatMessage({ id: "pages.users.sign-in.succeed" }),
            ],
          });
          // TODO
          dispatch(signIn({ token: res.token }));
          navigate(PERSONAL_PATH);
        })
        .catch((reason: IError[]) => {
          setAlert({
            color: "error",
            messages: reason.map((x) => x.message),
          });
        });
    },
  });
  return (
    <Layout
      title={intl.formatMessage({ id: "pages.users.sign-in.title" })}
      handleSubmit={formik.handleSubmit}
    >
      {alert && (
        <Alert severity={alert.color}>
          {alert.messages.map((x, i) => (
            <div key={i}>{x}</div>
          ))}
        </Alert>
      )}
      <FormControl>
        <FormLabel htmlFor="user">
          <FormattedMessage id="pages.users.sign-in.form.email-or-nickname.label" />
        </FormLabel>
        <TextField
          value={formik.values.user}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.user && Boolean(formik.errors.user)}
          helperText={formik.touched.user && formik.errors.user}
          color={formik.errors.user ? "error" : "primary"}
          type="text"
          name="user"
          autoFocus
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "user" }}
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
      <FormControlLabel
        control={
          <Checkbox
            value={formik.values.rememberMe}
            onChange={formik.handleChange}
            color="primary"
          />
        }
        label={
          <FormattedMessage id="pages.users.sign-in.form.email-or-nickname.label" />
        }
      />
      <Button type="submit" fullWidth variant="contained">
        <FormattedMessage id="buttons.submit" />
      </Button>
    </Layout>
  );
};

export default Widget;
