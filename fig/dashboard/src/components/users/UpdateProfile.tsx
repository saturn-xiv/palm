import { useState } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import { FormattedMessage, useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";
import moment from "moment-timezone";

import { useAppSelector, useAppDispatch } from "../../hooks";
import {
  currentUser as selectCurrentUser,
  updateProfile,
} from "../../reducers/current-user";
import { IErrorMessage } from "../../api/graphql";
import { update_profile } from "../../api/camelia";
import MessageBox, { IState as IMessageBox } from "../../components/MessageBox";
import { siteInfo as selectSiteInfo } from "../../reducers/site-info";

const validationSchema = yup_object({
  real_name: yup_string().required().min(2).max(31),
  avatar: yup_string().url().required().max(255),
  lang: yup_string().required(),
  timezone: yup_string().required(),
});

function Widget() {
  const current_user = useAppSelector(selectCurrentUser);
  const site_info = useAppSelector(selectSiteInfo);
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });
  const intl = useIntl();
  const dispatch = useAppDispatch();

  const formik = useFormik({
    enableReinitialize: true,
    initialValues: {
      real_name: current_user.realName,
      avatar: current_user.avatar,
      lang: current_user.lang,
      timezone: current_user.timezone,
    },
    validationSchema,

    onSubmit: (values) => {
      update_profile(
        values.real_name,
        values.avatar,
        values.lang,
        values.timezone
      )
        .then(() => {
          setMessageBox({
            messages: [intl.formatMessage({ id: "flashes.succeed" })],
            severity: "success",
          });
          dispatch(
            updateProfile({
              realName: values.real_name,
              avatar: values.avatar,
              lang: values.lang,
              timezone: values.timezone,
            })
          );
        })
        .catch((reason: IErrorMessage[]) => {
          setMessageBox({
            messages: reason.map((x) => x.message),
            severity: "error",
          });
        });
    },
  });

  return (
    <>
      <MessageBox
        severity={messageBox.severity}
        messages={messageBox.messages}
        handleClose={() => {
          setMessageBox({ messages: [], severity: "info" });
        }}
      />
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="users.update-profile.title" />
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
          label={intl.formatMessage({ id: "form.fields.avatar.label" })}
          name="avatar"
          value={formik.values.avatar}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.avatar && Boolean(formik.errors.avatar)}
          helperText={formik.touched.avatar && formik.errors.avatar}
        />
        <FormControl margin="normal" required fullWidth>
          <InputLabel id="user-profile-lang-select-label">
            <FormattedMessage id="form.fields.language.label" />
          </InputLabel>
          <Select
            labelId="user-profile-lang-select-label"
            name="lang"
            value={formik.values.lang}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          >
            {site_info.languages.map((x, i) => (
              <MenuItem key={i} value={x}>
                <FormattedMessage id={`languages.${x}`} />
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <FormControl margin="normal" required fullWidth>
          <InputLabel id="user-profile-timezone-select-label">
            <FormattedMessage id="form.fields.timezone.label" />
          </InputLabel>
          <Select
            labelId="user-profile-timezone-select-label"
            name="timezone"
            value={formik.values.timezone}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          >
            {moment.tz.names().map((x, i) => (
              <MenuItem key={i} value={x}>
                {x}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
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
