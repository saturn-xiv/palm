import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { sign_up_by_email } from "../../api/users";
import { IErrorMessage } from "../../api";
import { USERS_SIGN_IN_PATH } from "../../Router";

export const PASSWORD_MIN_LENGTH = 6;
export const PASSWORD_MAX_LENGTH = 31;
export const NICKNAME_MIN_LENGTH = 2;
export const NICKNAME_MAX_LENGTH = 31;
export const REAL_NAME_MIN_LENGTH = 2;
export const REAL_NAME_MAX_LENGTH = 63;
export const EMAIL_MAX_LENGTH = 127;

interface IForm {
  email: string;
  nickname: string;
  real_name: string;
  password: string;
  password_confirmation: string;
}

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();

  return (
    <Card title={<FormattedMessage id="users.sign-up.title" />} hoverable>
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
          sign_up_by_email(
            values.real_name,
            values.nickname,
            values.email,
            values.password
          )
            .then(() => {
              messageApi.success(
                intl.formatMessage({ id: "users.confirm.by-email.succeed" })
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
          name="real_name"
          label={<FormattedMessage id="form.fields.real-name.label" />}
          rules={[
            {
              required: true,
              min: REAL_NAME_MIN_LENGTH,
              max: REAL_NAME_MAX_LENGTH,
            },
          ]}
        />
        <ProFormText
          width="md"
          name="nickname"
          label={<FormattedMessage id="form.fields.nickname.label" />}
          rules={[
            {
              required: true,
              min: NICKNAME_MIN_LENGTH,
              max: NICKNAME_MAX_LENGTH,
            },
          ]}
        />
        <ProFormText
          width="md"
          name="email"
          label={<FormattedMessage id="form.fields.email.label" />}
          rules={[{ type: "email", required: true, max: EMAIL_MAX_LENGTH }]}
        />
        <ProFormText.Password
          width="md"
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
          width="md"
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
