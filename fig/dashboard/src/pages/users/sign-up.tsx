import TextField from "@mui/material/TextField";
import HowToRegOutlinedIcon from "@mui/icons-material/HowToRegOutlined";
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
      icon={<HowToRegOutlinedIcon />}
      handleSubmit={handleSubmit}
      title="users.sign-up.title"
    >
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.real-name.label" })}
        name="real_name"
        autoFocus
      />
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.nickname.label" })}
        name="nickname"
      />
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.email.label" })}
        name="email"
      />
      <TextField
        margin="normal"
        required
        fullWidth
        name="password"
        label={intl.formatMessage({ id: "form.fields.password.label" })}
        type="password"
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
      />
    </AnonymousForm>
  );
};
