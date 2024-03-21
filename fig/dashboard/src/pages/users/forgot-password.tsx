import { useState } from "react";
import { useFormik } from "formik";
import { string as yup_string, object as yup_object } from "yup";
import TextField from "@mui/material/TextField";
import { useIntl } from "react-intl";
import PasswordOutlinedIcon from "@mui/icons-material/PasswordOutlined";
import { useNavigate } from "react-router-dom";

import AnonymousForm from "../../layouts/application/Form";
import { forgot_password } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import MessageBox, { IState as IMessageBox } from "../../components/MessageBox";
import { SIGN_IN_PATH } from "../../reducers/current-user";

const validationSchema = yup_object({
  user: yup_string().required(),
});

export const Component = () => {
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });

  const intl = useIntl();
  const navigate = useNavigate();
  const formik = useFormik({
    initialValues: {
      user: "",
    },
    validationSchema,
    onSubmit: (values) => {
      forgot_password(values.user)
        .then(() => {
          setMessageBox({
            messages: [
              intl.formatMessage({ id: "users.forgot-password.succeed" }),
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
        title="users.forgot-password.title"
      >
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.account.label" })}
          name="user"
          value={formik.values.user}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.user && Boolean(formik.errors.user)}
          helperText={formik.touched.user && formik.errors.user}
        />
      </AnonymousForm>
    </>
  );
};
