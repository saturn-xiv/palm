import { Button, Popconfirm, message } from "antd";
import { LogoutOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { header_button_style } from "./style";
import { useAppDispatch } from "../../hooks";
import { SIGN_IN_PATH, signOut } from "../../reducers/current-user";
import { IError } from "../../api";
import { user_sign_out } from "../../api/daffodil";

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const dispatch = useAppDispatch();
  const intl = useIntl();
  const navigate = useNavigate();
  return (
    <Popconfirm
      title={<FormattedMessage id="pages.users.sign-out.title" />}
      description={<FormattedMessage id="pages.users.sign-out.description" />}
      onConfirm={() => {
        user_sign_out()
          .then(() => {
            messageApi
              .open({
                type: "success",
                content: intl.formatMessage({
                  id: "pages.users.sign-out.succeed",
                }),
                duration: 1,
              })
              .then(() => {
                dispatch(signOut());
                navigate(SIGN_IN_PATH);
              });
          })
          .catch((reason: IError[]) => {
            messageApi
              .error(reason.map((x) => x.message).join("\n"))
              .then(() => {
                navigate(SIGN_IN_PATH);
              });
          });
      }}
      okText={<FormattedMessage id="buttons.yes" />}
      cancelText={<FormattedMessage id="buttons.no" />}
    >
      {contextHolder}
      <Button
        style={header_button_style}
        type="text"
        icon={<LogoutOutlined />}
      />
    </Popconfirm>
  );
};

export default Widget;
