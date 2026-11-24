import { useEffect, useState } from "react";
import { useParams } from "react-router";
import Alert from "@mui/material/Alert";
import CheckIcon from "@mui/icons-material/Check";
import { useIntl } from "react-intl";

import type { IAlertMessage } from "../../..";
import SharedLinksByEmail from "../../SharedLinks";

const Widget = () => {
  const intl = useIntl();
  const [message, setMessage] = useState<IAlertMessage>();
  const { token } = useParams();
  useEffect(() => {
    const call = async () => {
      console.log(token);
      setMessage({
        severity: "success",
        content: intl.formatMessage({
          id: "portal.pages.anonymous.users.by-email.unlock.succeed",
        }),
      });
    };
    call();
  }, [token, intl]);
  return (
    <>
      {message && (
        <Alert
          icon={<CheckIcon fontSize="inherit" />}
          severity={message.severity}
        >
          {message.content}
        </Alert>
      )}
      <SharedLinksByEmail />
    </>
  );
};

export default Widget;
