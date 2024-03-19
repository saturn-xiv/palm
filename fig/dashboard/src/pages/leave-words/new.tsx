import TextField from "@mui/material/TextField";
import CommentOutlinedIcon from "@mui/icons-material/CommentOutlined";
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
      icon={<CommentOutlinedIcon />}
      title="leave-words.new.title"
      handleSubmit={handleSubmit}
    >
      <TextField
        margin="normal"
        required
        fullWidth
        label={intl.formatMessage({ id: "form.fields.content.label" })}
        name="content"
        multiline
        rows={4}
        autoFocus
      />
    </AnonymousForm>
  );
};
