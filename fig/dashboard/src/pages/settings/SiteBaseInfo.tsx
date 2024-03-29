import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";

import { IErrorMessage } from "../../api/graphql";
import { useAppDispatch } from "../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";
import { set_site_info } from "../../api/camelia";

interface IProps {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
  handleRefresh: () => void;
}

const validationSchema = yup_object({
  title: yup_string().required().min(1).max(63),
  subhead: yup_string().required().min(1).max(31),
  description: yup_string().required().min(1).max(511),
  copyright: yup_string().required().min(1).max(63),
});

const Widget = ({
  title,
  subhead,
  description,
  copyright,
  handleRefresh,
}: IProps) => {
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const formik = useFormik({
    enableReinitialize: true,
    initialValues: {
      title,
      subhead,
      description,
      copyright,
    },
    validationSchema,

    onSubmit: (values) => {
      set_site_info(
        values.title,
        values.subhead,
        values.description,
        values.copyright
      )
        .then(() => {
          handleRefresh();
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
      <Typography variant="h6" gutterBottom>
        <FormattedMessage id="settings.info.base.title" />
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
          label={intl.formatMessage({ id: "form.fields.title.label" })}
          name="title"
          value={formik.values.title}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.title && Boolean(formik.errors.title)}
          helperText={formik.touched.title && formik.errors.title}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.subhead.label" })}
          name="subhead"
          value={formik.values.subhead}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.subhead && Boolean(formik.errors.subhead)}
          helperText={formik.touched.subhead && formik.errors.subhead}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.description.label" })}
          name="description"
          multiline
          rows={4}
          value={formik.values.description}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={
            formik.touched.description && Boolean(formik.errors.description)
          }
          helperText={formik.touched.description && formik.errors.description}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({
            id: "settings.info.base.fields.copyright.label",
          })}
          name="copyright"
          value={formik.values.copyright}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.copyright && Boolean(formik.errors.copyright)}
          helperText={formik.touched.copyright && formik.errors.copyright}
        />

        <Button type="submit" variant="contained" sx={{ mt: 3, mb: 2 }}>
          <FormattedMessage id="buttons.submit" />
        </Button>
      </Box>
    </>
  );
};
export default Widget;
