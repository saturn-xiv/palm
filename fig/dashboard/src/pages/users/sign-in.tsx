import TextField from "@mui/material/TextField";
import FormControlLabel from "@mui/material/FormControlLabel";
import Checkbox from "@mui/material/Checkbox";
import LoginOutlinedIcon from "@mui/icons-material/LoginOutlined";
import { useIntl } from "react-intl";

import AnonymousForm from "../../layouts/application/Form";

export const Component = () => {
  const intl = useIntl();
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const data = new FormData(event.currentTarget);
    console.log({
      email: data.get("email"),
      password: data.get("password"),
    });
  };

  return (
    <AnonymousForm
      icon={<LoginOutlinedIcon />}
      title="users.sign-in.title"
      handleSubmit={handleSubmit}
    >
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.account.label" })}
        name="account"
        autoComplete="account"
        autoFocus
      />
      <TextField
        margin="normal"
        required
        fullWidth
        name="password"
        label={intl.formatMessage({ id: "form.fields.password.label" })}
        type="password"
        autoComplete="current-password"
      />
      <FormControlLabel
        control={<Checkbox value="remember" color="primary" />}
        label={intl.formatMessage({ id: "form.fields.remember-me.label" })}
      />
    </AnonymousForm>
  );
};
