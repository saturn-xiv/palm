import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useIntl } from "react-intl";

import { unlock_by_token } from "../../../api/camelia";
import MessageBox, {
  IState as IMessageBox,
} from "../../../components/MessageBox";
import { IErrorMessage } from "../../../api/graphql";
import { SIGN_IN_PATH } from "../../../reducers/current-user";

export function Component() {
  const [messageBox, setMessageBox] = useState<IMessageBox>({
    messages: [],
    severity: "info",
  });
  const { token } = useParams();
  const intl = useIntl();
  const navigate = useNavigate();

  useEffect(() => {
    unlock_by_token(token || "")
      .then(() => {
        setMessageBox({
          messages: [
            intl.formatMessage({ id: "users.unlock.by-token.succeed" }),
          ],
          severity: "success",
        });
      })
      .catch((reason: IErrorMessage[]) => {
        setMessageBox({
          messages: reason.map((x) => x.message),
          severity: "error",
        });
      });
  });
  return (
    <MessageBox
      severity={messageBox.severity}
      messages={messageBox.messages}
      handleClose={() => {
        setMessageBox({ messages: [], severity: "info" });
        navigate(SIGN_IN_PATH);
      }}
    />
  );
}
