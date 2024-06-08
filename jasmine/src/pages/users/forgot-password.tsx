import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { forgot_password } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import { USERS_SIGN_IN_PATH } from "../../Router";

interface IForm {
  user: string;
}

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();

  return (
    <Card
      title={<FormattedMessage id="users.forgot-password.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          forgot_password(values.user)
            .then(() => {
              messageApi.success(
                intl.formatMessage({ id: "users.forgot-password.succeed" })
              );
              navigate(USERS_SIGN_IN_PATH);
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
      >
        <ProFormText
          width="md"
          name="user"
          label={<FormattedMessage id="form.fields.account.label" />}
          rules={[{ required: true }]}
        />
      </ProForm>
    </Card>
  );
};
