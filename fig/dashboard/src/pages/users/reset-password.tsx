import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate, useParams } from "react-router-dom";

import { SIGN_IN_PATH } from "../../reducers/current-user";
import { reset_email_user_password_by_token } from "../../api/daffodil";
import { IError } from "../../api";
import { PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH } from "../../components";

interface IFormValue {
  password: string;
  passwordConfirmation: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  const { token } = useParams<{ token: string }>();
  const navigate = useNavigate();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.reset-password.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          reset_email_user_password_by_token(token || "", values.password)
            .then(() => {
              messageApi
                .success(
                  intl.formatMessage({
                    id: "pages.users.reset-password.succeed",
                  })
                )
                .then(() => {
                  navigate(SIGN_IN_PATH);
                });
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="users.reset-password"
        request={async () => {
          return {
            password: "",
            passwordConfirmation: "",
          };
        }}
        autoFocusFirstInput
      >
        <ProFormText.Password
          name="password"
          width="md"
          label={<FormattedMessage id="form.fields.password.label" />}
          rules={[
            { required: true },
            { min: PASSWORD_MIN_LENGTH, max: PASSWORD_MAX_LENGTH },
          ]}
        />
        <ProFormText.Password
          name="passwordConfirmation"
          width="md"
          label={
            <FormattedMessage id="form.fields.password-confirmation.label" />
          }
          rules={[
            { required: true },
            ({ getFieldValue }) => ({
              validator(_, value) {
                if (!value || getFieldValue("password") === value) {
                  return Promise.resolve();
                }
                return Promise.reject(
                  new Error(
                    intl.formatMessage({
                      id: "form.fields.password-confirmation.errors.not-match",
                    })
                  )
                );
              },
            }),
          ]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
