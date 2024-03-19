import TextField from "@mui/material/TextField";
import { useIntl } from "react-intl";
import VerifiedUserOutlinedIcon from "@mui/icons-material/VerifiedUserOutlined";

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
      icon={<VerifiedUserOutlinedIcon />}
      handleSubmit={handleSubmit}
      title="users.confirm.by-email.title"
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
