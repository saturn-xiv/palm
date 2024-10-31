import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useIntl } from "react-intl";
import Typography from "@mui/material/Typography";
import Alert from "@mui/material/Alert";

import { Card } from "../../../layouts/sign-in-side/Card";
import { IAlert } from "../../../components";
import { SIGN_IN_PATH } from "../../../reducers/current-user";
import { unlock_email_user_by_token } from "../../../api/daffodil";
import { IError } from "../../../api";
import NotFound from "../../../components/NotFound";

const Widget = () => {
  const intl = useIntl();
  const [alert, setAlert] = useState<IAlert>();
  const { token } = useParams<{ token: string }>();
  const navigate = useNavigate();

  useEffect(() => {
    if (token) {
      unlock_email_user_by_token(token)
        .then(() => {
          setAlert({
            color: "success",
            messages: [
              intl.formatMessage({ id: "pages.users.unlock.succeed" }),
            ],
          });
        })
        .catch((reason: IError[]) => {
          setAlert({
            color: "error",
            messages: reason.map((x) => x.message),
          });
        });
    }
  }, [intl, token]);
  return token ? (
    <Card variant="outlined">
      <Typography
        component="h1"
        variant="h4"
        sx={{ width: "100%", fontSize: "clamp(2rem, 10vw, 2.15rem)" }}
      >
        {intl.formatMessage({ id: "pages.users.unlock.title" })}
      </Typography>
      {alert && (
        <Alert
          severity={alert.color}
          onClose={() => {
            if (alert.color == "success") {
              navigate(SIGN_IN_PATH);
            }
          }}
        >
          {alert.messages.map((x, i) => (
            <div key={i}>{x}</div>
          ))}
        </Alert>
      )}
    </Card>
  ) : (
    <NotFound />
  );
};

export default Widget;
