import { useEffect } from "react";
import { Card, message } from "antd";
import { useIntl, FormattedMessage } from "react-intl";
import { useNavigate, useParams } from "react-router-dom";

import { unlock_by_email_token } from "../../../api/users";
import { IErrorMessage } from "../../../api/graphql";
import { useAppDispatch } from "../../../hooks";
import { USERS_SIGN_IN_PATH } from "../../../Router";

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();
  const { token } = useParams();
  const dispatch = useAppDispatch();

  useEffect(() => {
    if (token) {
      unlock_by_email_token(token)
        .then(() => {
          messageApi.info({
            type: "success",
            content: intl.formatMessage({
              id: "users.unlock.by-token.succeed",
            }),
            onClose: () => {
              navigate(USERS_SIGN_IN_PATH);
            },
          });
        })
        .catch((reason: IErrorMessage[]) => {
          messageApi.error(reason.map((x) => x.message).join("\n"));
        });
    }
  }, [dispatch, intl, messageApi, navigate, token]);

  return (
    <Card
      title={<FormattedMessage id="users.unlock.by-token.title" />}
      hoverable
    >
      {contextHolder}
    </Card>
  );
};
