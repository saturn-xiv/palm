import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import FormControlLabel from "@mui/material/FormControlLabel";
import Checkbox from "@mui/material/Checkbox";
import LoginOutlinedIcon from "@mui/icons-material/LoginOutlined";
import { useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";
import { useNavigate } from "react-router-dom";

import AnonymousForm from "../../layouts/application/Form";
import { sign_in_by_email } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import { PERSONAL_PATH } from "../../reducers/current-user";
import { useAppDispatch } from "../../hooks";
import { signIn } from "../../reducers/current-user";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";

const validationSchema = yup_object({
  user: yup_string().required(),
  password: yup_string().required(),
});

export const Component = () => {
  const dispatch = useAppDispatch();

  const intl = useIntl();
  const navigate = useNavigate();
  const formik = useFormik({
    initialValues: {
      user: "",
      password: "",
    },
    validationSchema,
    onSubmit: (values) => {
      sign_in_by_email(values.user, values.password)
        .then((res) => {
          dispatch(signIn(res));
          dispatch(
            success_box([intl.formatMessage({ id: "users.sign-in.succeed" })])
          );
          navigate(PERSONAL_PATH);
        })
        .catch((reason: IErrorMessage[]) => {
          dispatch(error_box(reason.map((x) => x.message)));
        });
    },
  });

  return (
    <AnonymousForm
      icon={<LoginOutlinedIcon />}
      title="users.sign-in.title"
      handleSubmit={formik.handleSubmit}
    >
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.account.label" })}
        name="user"
        autoComplete="user"
        value={formik.values.user}
        onChange={formik.handleChange}
        onBlur={formik.handleBlur}
        error={formik.touched.user && Boolean(formik.errors.user)}
        helperText={formik.touched.user && formik.errors.user}
      />
      <TextField
        margin="normal"
        required
        fullWidth
        name="password"
        label={intl.formatMessage({ id: "form.fields.password.label" })}
        type="password"
        value={formik.values.password}
        onChange={formik.handleChange}
        onBlur={formik.handleBlur}
        error={formik.touched.password && Boolean(formik.errors.password)}
        helperText={formik.touched.password && formik.errors.password}
      />
      <FormControlLabel
        control={<Checkbox value="remember" color="primary" />}
        label={intl.formatMessage({ id: "form.fields.remember-me.label" })}
      />
    </AnonymousForm>
  );
};
