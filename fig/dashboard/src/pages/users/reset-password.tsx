import { useState } from "react";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import { useIntl } from "react-intl";
import PasswordOutlinedIcon from "@mui/icons-material/PasswordOutlined";
import { useParams, useNavigate } from "react-router-dom";
import {
  string as yup_string,
  ref as yup_ref,
  object as yup_object,
} from "yup";

import AnonymousForm from "../../layouts/application/Form";
import { reset_password } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import MessageBox, { IState as IMessageBox } from "../../components/MessageBox";
import {
  SIGN_IN_PATH,
  MAX_PASSWORD_LENGTH,
  MIN_PASSWORD_LENGTH,
} from "../../reducers/current-user";

const validationSchema = yup_object({
  password: yup_string()
    .required()
    .min(MIN_PASSWORD_LENGTH)
    .max(MAX_PASSWORD_LENGTH),
  password_confirmation: yup_string().oneOf([yup_ref("password")]),
});

export const Component = () => {
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });
  const { token } = useParams();
  const intl = useIntl();
  const navigate = useNavigate();

  const formik = useFormik({
    initialValues: {
      password: "",
      password_confirmation: "",
    },
    validationSchema,
    onSubmit: (values) => {
      reset_password(token || "", values.password)
        .then(() => {
          setMessageBox({
            messages: [
              intl.formatMessage({ id: "users.reset-password.succeed" }),
            ],
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
          navigate(SIGN_IN_PATH);
        }}
      />
      <AnonymousForm
        icon={<PasswordOutlinedIcon />}
        handleSubmit={formik.handleSubmit}
        title="users.reset-password.title"
      >
        <TextField
          margin="normal"
          required
          fullWidth
          name="password"
          label={intl.formatMessage({ id: "form.fields.password.label" })}
          type="password"
          autoFocus
          value={formik.values.password}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.password && Boolean(formik.errors.password)}
          helperText={formik.touched.password && formik.errors.password}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          name="password_confirmation"
          label={intl.formatMessage({
            id: "form.fields.password-confirmation.label",
          })}
          type="password"
          value={formik.values.password_confirmation}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={
            formik.touched.password_confirmation &&
            Boolean(formik.errors.password_confirmation)
          }
          helperText={
            formik.touched.password_confirmation &&
            formik.errors.password_confirmation
          }
        />
      </AnonymousForm>
    </>
  );
};
