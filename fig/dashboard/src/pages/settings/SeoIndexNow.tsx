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
import {
  set_index_now_site_verification,
  get_index_now_site_verification,
} from "../../api/camelia";

const validationSchema = yup_object({
  key: yup_string().required().min(1).max(127),
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
      key: "",
    },
    validationSchema,
    onSubmit: (form) => {
      set_index_now_site_verification(form.key)
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
    get_index_now_site_verification().then((res) => {
      setFieldValue("key", res.key);
    });
  }, [setFieldValue]);
  return (
    <>
      <Typography variant="h6" gutterBottom>
        <FormattedMessage id="settings.seo.index-now.title" />
      </Typography>
      <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({
            id: "form.fields.key.label",
          })}
          name="key"
          value={values.key}
          onChange={handleChange}
          onBlur={handleBlur}
          error={touched.key && Boolean(errors.key)}
          helperText={touched.key && errors.key}
        />

        <Button type="submit" variant="contained" sx={{ mt: 3, mb: 2 }}>
          <FormattedMessage id="buttons.submit" />
        </Button>
      </Box>
    </>
  );
};
export default Widget;
