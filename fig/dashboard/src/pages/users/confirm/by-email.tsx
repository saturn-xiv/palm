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

import Layout from "../../../layouts/sign-in-side/Card";
import { IAlert } from "../../../components";
import { SIGN_IN_PATH } from "../../../reducers/current-user";
import { user_confirm_by_email } from "../../../api/daffodil";
import { IError } from "../../../api";

interface IByEmailFormProps {
  title: string;
  handleSubmit: (values: IByEmailFormValues) => void;
  alert?: IAlert;
}

export interface IByEmailFormValues {
  user: string;
}

const validationSchema = yup.object({
  user: yup.string().trim().required(),
});

export const ByEmailForm = ({
  title,
  alert,
  handleSubmit,
}: IByEmailFormProps) => {
  const navigate = useNavigate();
  const initialValues: IByEmailFormValues = {
    user: "",
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
          name="realName"
          autoFocus
          required
          fullWidth
          variant="outlined"
          sx={{ ariaLabel: "realName" }}
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
  const handleSubmit = (values: IByEmailFormValues) => {
    user_confirm_by_email(values.user)
      .then(() => {
        setAlert({
          color: "success",
          messages: [
            intl.formatMessage({ id: "pages.users.confirm.instruction" }),
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
    <ByEmailForm
      title={intl.formatMessage({ id: "pages.users.confirm.title" })}
      alert={alert}
      handleSubmit={handleSubmit}
    />
  );
};
export default Widget;
