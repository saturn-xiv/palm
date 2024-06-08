import { useEffect } from "react";
import { Card, message } from "antd";
import { useIntl, FormattedMessage } from "react-intl";
import { useNavigate, useParams } from "react-router-dom";

import { unlock_by_token } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { useAppDispatch } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import {
  USERS_SIGN_IN_PATH,
  USERS_UNLOCK_BY_TOKEN_PATH,
} from "../../../Router";

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();
  const { token } = useParams();
  const dispatch = useAppDispatch();

  useEffect(() => {
    dispatch(set_pathname(USERS_UNLOCK_BY_TOKEN_PATH));
    if (token) {
      unlock_by_token(token)
        .then(() => {
          messageApi.success(
            intl.formatMessage({ id: "users.unlock.by-token.succeed" })
          );
          navigate(USERS_SIGN_IN_PATH);
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
