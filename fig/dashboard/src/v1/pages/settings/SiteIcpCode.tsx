import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import { FormattedMessage, useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";
import { useConfirm } from "material-ui-confirm";

import { IErrorMessage } from "../../api/graphql";
import { useAppDispatch } from "../../hooks";
import {
  success as success_box,
  error as error_box,
} from "../../reducers/message-box";
import { delete_site_icp_code, set_site_icp_code } from "../../api/camelia";

interface IProps {
  code?: string;
  handleRefresh: () => void;
}

const validationSchema = yup_object({
  code: yup_string().required().min(1).max(63),
});

const Widget = ({ code, handleRefresh }: IProps) => {
  const confirm = useConfirm();
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const formik = useFormik({
    enableReinitialize: true,
    initialValues: {
      code: code || "",
    },
    validationSchema,

    onSubmit: (values) => {
      set_site_icp_code(values.code)
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
        <FormattedMessage id="settings.info.icp-code.title" />
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
          label={intl.formatMessage({ id: "form.fields.code.label" })}
          name="code"
          value={formik.values.code}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.code && Boolean(formik.errors.code)}
          helperText={formik.touched.code && formik.errors.code}
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
                delete_site_icp_code()
                  .then(() => {
                    dispatch(
                      success_box([
                        intl.formatMessage({ id: "flashes.succeed" }),
                      ])
                    );
                    handleRefresh();
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
