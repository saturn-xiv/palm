import { message, Typography } from "antd";
import { useParams, useNavigate } from "react-router-dom";
import { FormattedMessage, useIntl } from "react-intl";
import { useEffect } from "react";

import { SIGN_IN_PATH } from "../../../reducers/current-user";
import { unlock_email_user_by_token } from "../../../api/daffodil";
import { IError } from "../../../api";

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const { token } = useParams<{ token: string }>();
  const navigate = useNavigate();

  useEffect(() => {
    if (token) {
      unlock_email_user_by_token(token)
        .then(() => {
          messageApi
            .success(intl.formatMessage({ id: "pages.users.unlock.succeed" }))
            .then(() => {
              navigate(SIGN_IN_PATH);
            });
        })
        .catch((reason: IError[]) => {
          messageApi.error(reason.map((x) => x.message).join("\n"));
        });
    }
  }, [intl, token, messageApi, navigate]);
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.unlock.title" />
      </Typography.Title>
      {contextHolder}
    </>
  );
};

export default Widget;
