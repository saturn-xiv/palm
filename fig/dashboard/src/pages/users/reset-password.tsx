import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate, useParams } from "react-router-dom";

import { reset_password } from "../../api/camelia";
import { IErrorMessage } from "../../api/graphql";
import { USERS_SIGN_IN_PATH } from "../../Router";
import { PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH } from "./sign-up";

interface IForm {
  password: string;
  password_confirmation: string;
}

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();

  const { token } = useParams();
  const intl = useIntl();
  const navigate = useNavigate();

  return (
    <Card
      title={<FormattedMessage id="users.reset-password.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          if (values.password !== values.password_confirmation) {
            messageApi.error(
              intl.formatMessage({
                id: "form.fields.password-confirmation.error",
              })
            );
            return;
          }
          if (token) {
            reset_password(token, values.password)
              .then(() => {
                messageApi.success(
                  intl.formatMessage({ id: "users.reset-password.succeed" })
                );
                navigate(USERS_SIGN_IN_PATH);
              })
              .catch((reason: IErrorMessage[]) => {
                messageApi.error(reason.map((x) => x.message).join("\n"));
              });
          }
        }}
      >
        <ProFormText.Password
          width="sm"
          name="password"
          label={<FormattedMessage id="form.fields.password.label" />}
          rules={[
            {
              required: true,
              min: PASSWORD_MIN_LENGTH,
              max: PASSWORD_MAX_LENGTH,
            },
          ]}
        />
        <ProFormText.Password
          width="sm"
          name="password_confirmation"
          label={
            <FormattedMessage id="form.fields.password-confirmation.label" />
          }
          rules={[
            {
              required: true,
            },
          ]}
        />
      </ProForm>
    </Card>
  );
};
