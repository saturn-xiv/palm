import { useState } from "react";
import { useFormik } from "formik";
import { string as yup_string, object as yup_object } from "yup";
import TextField from "@mui/material/TextField";
import { useIntl } from "react-intl";
import LockOpenOutlinedIcon from "@mui/icons-material/LockOpenOutlined";

import AnonymousForm from "../../../layouts/application/Form";
import { unlock_by_email } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import MessageBox, {
  IState as IMessageBox,
} from "../../../components/MessageBox";

const validationSchema = yup_object({
  user: yup_string().required(),
});

export const Component = () => {
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });

  const intl = useIntl();
  const formik = useFormik({
    initialValues: {
      user: "",
    },
    validationSchema,
    onSubmit: (values) => {
      unlock_by_email(values.user)
        .then(() => {
          setMessageBox({
            messages: [
              intl.formatMessage({ id: "users.unlock.by-email.succeed" }),
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
        handleClose={() => setMessageBox({ messages: [], severity: "info" })}
      />
      <AnonymousForm
        icon={<LockOpenOutlinedIcon />}
        handleSubmit={formik.handleSubmit}
        title="users.unlock.by-email.title"
      >
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.account.label" })}
          name="user"
          autoFocus
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
