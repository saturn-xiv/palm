import { useIntl } from "react-intl";
import Typography from "@mui/material/Typography";

import Title from "../../../../layouts/Title";
import SharedLinksByEmail from "../SharedLinks";
import EmailForm from "./EmailForm";

const Widget = () => {
  const intl = useIntl();

  return (
    <>
      <Typography
        component="h1"
        variant="h4"
        sx={{ width: "100%", fontSize: "clamp(2rem, 10vw, 2.15rem)" }}
      >
        <Title
          value={intl.formatMessage({
            id: "portal.pages.anonymous.users.by-email.forgot-password.title",
          })}
        />
      </Typography>
      <EmailForm
        submit="portal.pages.anonymous.users.by-email.forgot-password.request"
        doSubmit={(email: string) => {
          // TODO
          console.log("forgot-password", email);
        }}
      />
      <SharedLinksByEmail />
    </>
  );
};

export default Widget;
