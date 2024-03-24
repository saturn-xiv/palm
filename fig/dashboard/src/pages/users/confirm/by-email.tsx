import { useFormik } from "formik";
import { string as yup_string, object as yup_object } from "yup";
import TextField from "@mui/material/TextField";
import { useIntl } from "react-intl";
import VerifiedUserOutlinedIcon from "@mui/icons-material/VerifiedUserOutlined";
import { useNavigate } from "react-router-dom";

import AnonymousForm from "../../../layouts/application/Form";
import { confirm_by_email } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { SIGN_IN_PATH } from "../../../reducers/current-user";
import { useAppDispatch } from "../../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../../reducers/message-box";

const validationSchema = yup_object({
  user: yup_string().required(),
});

export const Component = () => {
  const dispatch = useAppDispatch();

  const intl = useIntl();
  const navigate = useNavigate();
  const formik = useFormik({
    initialValues: {
      user: "",
    },
    validationSchema,
    onSubmit: (values) => {
      confirm_by_email(values.user)
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
      icon={<VerifiedUserOutlinedIcon />}
      handleSubmit={formik.handleSubmit}
      title="users.confirm.by-email.title"
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
  );
};
