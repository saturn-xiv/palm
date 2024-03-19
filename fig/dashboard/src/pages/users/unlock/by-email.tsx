import TextField from "@mui/material/TextField";
import { useIntl } from "react-intl";
import LockOpenOutlinedIcon from "@mui/icons-material/LockOpenOutlined";

import AnonymousForm from "../../../layouts/application/Form";

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
      icon={<LockOpenOutlinedIcon />}
      handleSubmit={handleSubmit}
      title="users.unlock.by-email.title"
    >
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.account.label" })}
        name="account"
        autoFocus
      />
    </AnonymousForm>
  );
};
