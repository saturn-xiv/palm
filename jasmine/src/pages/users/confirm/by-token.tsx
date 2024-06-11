import { useEffect } from "react";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate, useParams } from "react-router-dom";

import { confirm_by_email_token } from "../../../api/users";
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
      confirm_by_email_token(token)
        .then(() => {
          messageApi.info({
            type: "success",
            content: intl.formatMessage({
              id: "users.confirm.by-token.succeed",
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
      title={<FormattedMessage id="users.confirm.by-token.title" />}
      hoverable
    >
      {contextHolder}
    </Card>
  );
};
