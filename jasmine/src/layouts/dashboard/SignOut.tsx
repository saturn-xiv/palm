import { ReactNode } from "react";
import { Dropdown, Popconfirm, message } from "antd";
import { LogoutOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { useAppDispatch } from "../../hooks";
import { signOut, remove as remove_token } from "../../reducers/current-user";
import { sign_out } from "../../api/users";
import { IErrorMessage } from "../../api/graphql";
import { USERS_SIGN_IN_PATH } from "../../Router";

interface IProps {
  children: ReactNode;
}
const Widget = ({ children }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const dispatch = useAppDispatch();
  const intl = useIntl();
  const navigate = useNavigate();
  return (
    <Dropdown
      menu={{
        items: [
          {
            key: "logout",
            label: (
              <Popconfirm
                title={<FormattedMessage id="users.sign-out.title" />}
                description={<FormattedMessage id="users.sign-out.confirm" />}
                onConfirm={() => {
                  sign_out()
                    .then(() => {
                      remove_token();
                      messageApi.info({
                        type: "success",
                        content: intl.formatMessage({
                          id: "users.sign-out.succeed",
                        }),
                        onClose: () => {
                          dispatch(signOut());
                          navigate(USERS_SIGN_IN_PATH);
                        },
                        duration: 1,
                      });
                    })
                    .catch((reason: IErrorMessage[]) => {
                      message.error(reason.map((x) => x.message).join("\n"));
                    });
                }}
                okText={<FormattedMessage id="buttons.ok" />}
                cancelText={<FormattedMessage id="buttons.cancel" />}
              >
                {contextHolder}
                <LogoutOutlined />
                &nbsp;
                <FormattedMessage id="users.sign-out.title" />
              </Popconfirm>
            ),
          },
        ],
      }}
    >
      {children}
    </Dropdown>
  );
};

export default Widget;
