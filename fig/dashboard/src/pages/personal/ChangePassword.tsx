import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { change_email_user_password } from "../../api/daffodil";
import { IError } from "../../api";
import { PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH } from "../../components";

export interface IFormValue {
  currentPassword: string;
  newPassword: string;
  passwordConfirmation: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();

  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.change-password.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          change_email_user_password(values.currentPassword, values.newPassword)
            .then(() => {
              messageApi
                .success(
                  intl.formatMessage({
                    id: "pages.users.change-password.succeed",
                  })
                )
                .then(() => {
                  formRef.current?.resetFields();
                });
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="users.change-password"
        request={async () => {
          return {
            currentPassword: "",
            newPassword: "",
            passwordConfirmation: "",
          };
        }}
        autoFocusFirstInput
      >
        <ProFormText.Password
          name="currentPassword"
          width="md"
          label={<FormattedMessage id="form.fields.current-password.label" />}
          rules={[
            { required: true },
            { min: PASSWORD_MIN_LENGTH, max: PASSWORD_MAX_LENGTH },
          ]}
        />
        <ProFormText.Password
          name="newPassword"
          width="md"
          label={<FormattedMessage id="form.fields.new-password.label" />}
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
                if (!value || getFieldValue("newPassword") === value) {
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
