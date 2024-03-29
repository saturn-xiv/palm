import { useEffect } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import { useIntl, FormattedMessage } from "react-intl";
import { useFormik } from "formik";
import { string as yup_string, object as yup_object } from "yup";

import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";
import { useAppDispatch } from "../../hooks";
import { IErrorMessage } from "../../api/graphql";
import { get_google_recaptcha, set_google_recaptcha } from "../../api/camelia";

const validationSchema = yup_object({
  site_key: yup_string().required().min(1).max(127),
  secret: yup_string().required().min(1).max(127),
});

const Widget = () => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const {
    setFieldValue,
    values,
    errors,
    touched,
    handleSubmit,
    handleChange,
    handleBlur,
  } = useFormik({
    initialValues: {
      site_key: "",
      secret: "",
    },
    validationSchema,
    onSubmit: (form) => {
      set_google_recaptcha(form.site_key, form.secret)
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
  useEffect(() => {
    get_google_recaptcha().then((res) => {
      setFieldValue("site_key", res.siteKey);
      setFieldValue("secret", res.secret);
    });
  }, [setFieldValue]);
  return (
    <>
      <Typography variant="h6" gutterBottom>
        <FormattedMessage id="settings.seo.recaptcha.title" />
      </Typography>
      <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({
            id: "settings.seo.recaptcha.form.fields.site-key.label",
          })}
          name="site_key"
          value={values.site_key}
          onChange={handleChange}
          onBlur={handleBlur}
          error={touched.site_key && Boolean(errors.site_key)}
          helperText={touched.site_key && errors.site_key}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({
            id: "form.fields.secret.label",
          })}
          name="secret"
          value={values.secret}
          onChange={handleChange}
          onBlur={handleBlur}
          error={touched.secret && Boolean(errors.secret)}
          helperText={touched.secret && errors.secret}
        />
        <Button type="submit" variant="contained" sx={{ mt: 3, mb: 2 }}>
          <FormattedMessage id="buttons.submit" />
        </Button>
      </Box>
    </>
  );
};
export default Widget;
