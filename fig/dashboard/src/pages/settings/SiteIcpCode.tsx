import { useState } from "react";

import { IErrorMessage } from "../..api/graphql";
import { bill_form_options, update_bill, IBill } from "../..api/daffodil";
import { useAppDispatch } from "../..hooks";
import { ICurrencyOption } from "../..api/camelia";
import {
  success as success_box,
  error as error_box,
} from "../..reducers/message-box";

interface IProps {
  code?: string;
  handleRefresh: () => void;
}

const Widget = ({ code }: IProps) => {
  const [value, setValue] = useState<string | undefined>();
  return (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="users.update-profile.title" />
      </Typography>
      <Box component="form" onSubmit={(e) => {}} noValidate sx={{ mt: 1 }}>
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
};
export default Widget;
