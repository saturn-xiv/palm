import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import {
  string as yup_string,
  ref as yup_ref,
  object as yup_object,
} from "yup";

import {
  MAX_PASSWORD_LENGTH,
  MIN_PASSWORD_LENGTH,
} from "../../reducers/current-user";
import { IErrorMessage } from "../../api/graphql";
import { change_password } from "../../api/camelia";
import { useAppDispatch } from "../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";

const validationSchema = yup_object({
  current_password: yup_string().required(),
  new_password: yup_string()
    .required()
    .min(MIN_PASSWORD_LENGTH)
    .max(MAX_PASSWORD_LENGTH),
  password_confirmation: yup_string().oneOf([yup_ref("new_password")]),
});

function Widget() {
  const dispatch = useAppDispatch();
  const intl = useIntl();

  const formik = useFormik({
    initialValues: {
      current_password: "",
      new_password: "",
      password_confirmation: "",
    },
    validationSchema,
    onSubmit: (values) => {
      change_password(values.current_password, values.new_password)
        .then(() => {
          dispatch(
            success_box([intl.formatMessage({ id: "flashes.succeed" })])
          );
        })
        .catch((reason: IErrorMessage[]) => {
          dispatch(error_box(reason.map((x) => x.message)));
        });
    },
  });
  return (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="users.change-password.title" />
      </Typography>
      <Box
        component="form"
        onSubmit={formik.handleSubmit}
        noValidate
        sx={{ mt: 1 }}
      >
        <TextField
          margin="normal"
          required
          fullWidth
          name="current_password"
          label={intl.formatMessage({
            id: "form.fields.current-password.label",
          })}
          type="password"
          value={formik.values.current_password}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={
            formik.touched.current_password &&
            Boolean(formik.errors.current_password)
          }
          helperText={
            formik.touched.current_password && formik.errors.current_password
          }
        />
        <TextField
          margin="normal"
          required
          fullWidth
          name="new_password"
          label={intl.formatMessage({ id: "form.fields.new-password.label" })}
          type="password"
          value={formik.values.new_password}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={
            formik.touched.new_password && Boolean(formik.errors.new_password)
          }
          helperText={formik.touched.new_password && formik.errors.new_password}
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
        <Button
          type="submit"
          fullWidth
          variant="contained"
          sx={{ mt: 3, mb: 2 }}
        >
          <FormattedMessage id="buttons.submit" />
        </Button>
      </Box>
    </>
  );
}

export default Widget;
