import { useEffect } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import { useIntl, FormattedMessage } from "react-intl";
import { useFormik } from "formik";
import { useConfirm } from "material-ui-confirm";
import { string as yup_string, object as yup_object } from "yup";

import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";
import { useAppDispatch } from "../../hooks";
import { IErrorMessage } from "../../api/graphql";
import {
  delete_google_site_verification,
  get_google_site_verification,
  set_google_site_verification,
} from "../../api/camelia";

const validationSchema = yup_object({
  code: yup_string().required().min(1).max(127),
});

const Widget = () => {
  const confirm = useConfirm();
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
      code: "",
    },
    validationSchema,
    onSubmit: (form) => {
      set_google_site_verification(form.code)
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
    get_google_site_verification().then((res) => {
      setFieldValue("code", res.code);
    });
  }, [setFieldValue]);
  return (
    <>
      <Typography variant="h6" gutterBottom>
        <FormattedMessage id="settings.seo.google.title" />
      </Typography>
      <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({
            id: "form.fields.code.label",
          })}
          name="code"
          value={values.code}
          onChange={handleChange}
          onBlur={handleBlur}
          error={touched.code && Boolean(errors.code)}
          helperText={touched.code && errors.code}
        />
        <Button type="submit" variant="contained" sx={{ mt: 3, mb: 2 }}>
          <FormattedMessage id="buttons.submit" />
        </Button>
        <Button
          color="error"
          variant="text"
          sx={{ mt: 3, mb: 2 }}
          onClick={() => {
            confirm({
              title: intl.formatMessage({
                id: "flashes.are-you-sure",
              }),
            })
              .then(() => {
                delete_google_site_verification()
                  .then(() => {
                    dispatch(
                      success_box([
                        intl.formatMessage({ id: "flashes.succeed" }),
                      ])
                    );
                    setFieldValue("code", "");
                  })
                  .catch((reason: IErrorMessage[]) => {
                    dispatch(error_box(reason.map((x) => x.message)));
                  });
              })
              .catch(() => {});
          }}
        >
          <FormattedMessage id="buttons.delete" />
        </Button>
      </Box>
    </>
  );
};
export default Widget;
