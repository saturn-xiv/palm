import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import HowToRegOutlinedIcon from "@mui/icons-material/HowToRegOutlined";
import { useIntl } from "react-intl";
import {
  string as yup_string,
  ref as yup_ref,
  object as yup_object,
} from "yup";
import { useNavigate } from "react-router-dom";

import AnonymousForm from "../../layouts/application/Form";
import { sign_up_by_email } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import {
  SIGN_IN_PATH,
  MAX_PASSWORD_LENGTH,
  MIN_PASSWORD_LENGTH,
} from "../../reducers/current-user";
import { useAppDispatch } from "../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";

const validationSchema = yup_object({
  email: yup_string().email().required().min(5).max(127),
  real_name: yup_string().required().min(2).max(63),
  nickname: yup_string().required().min(2).max(31),
  password: yup_string()
    .required()
    .min(MIN_PASSWORD_LENGTH)
    .max(MAX_PASSWORD_LENGTH),
  password_confirmation: yup_string().oneOf([yup_ref("password")]),
});

export const Component = () => {
  const dispatch = useAppDispatch();

  const intl = useIntl();
  const navigate = useNavigate();
  const formik = useFormik({
    initialValues: {
      real_name: "",
      nickname: "",
      email: "",
      password: "",
      password_confirmation: "",
    },
    validationSchema,
    onSubmit: (values) => {
      sign_up_by_email(
        values.real_name,
        values.nickname,
        values.email,
        values.password
      )
        .then(() => {
          dispatch(
            success_box([
              intl.formatMessage({ id: "users.confirm.by-email.succeed" }),
            ])
          );
          navigate(SIGN_IN_PATH);
        })
        .catch((reason: IErrorMessage[]) => {
          dispatch(error_box(reason.map((x) => x.message)));
        });
    },
  });

  return (
    <AnonymousForm
      icon={<HowToRegOutlinedIcon />}
      handleSubmit={formik.handleSubmit}
      title="users.sign-up.title"
    >
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.real-name.label" })}
        name="real_name"
        value={formik.values.real_name}
        onChange={formik.handleChange}
        onBlur={formik.handleBlur}
        error={formik.touched.real_name && Boolean(formik.errors.real_name)}
        helperText={formik.touched.real_name && formik.errors.real_name}
      />
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.nickname.label" })}
        name="nickname"
        value={formik.values.nickname}
        onChange={formik.handleChange}
        onBlur={formik.handleBlur}
        error={formik.touched.nickname && Boolean(formik.errors.nickname)}
        helperText={formik.touched.nickname && formik.errors.nickname}
      />
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.email.label" })}
        name="email"
        value={formik.values.email}
        onChange={formik.handleChange}
        onBlur={formik.handleBlur}
        error={formik.touched.email && Boolean(formik.errors.email)}
        helperText={formik.touched.email && formik.errors.email}
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
  );
};
