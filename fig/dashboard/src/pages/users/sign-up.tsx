import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { guess_timezone } from "../../utils";
import { user_sign_up_by_email } from "../../api/daffodil";
import { IError } from "../../api";
import { SIGN_IN_PATH } from "../../reducers/current-user";
import {
  EMAIL_MAX_LENGTH,
  EMAIL_MIN_LENGTH,
  NAME_MAX_LENGTH,
  NAME_MIN_LENGTH,
  PASSWORD_MAX_LENGTH,
  PASSWORD_MIN_LENGTH,
} from "../../components";

export interface IFormValue {
  realName: string;
  nickname: string;
  email: string;
  password: string;
  passwordConfirmation: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  const navigate = useNavigate();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.sign-up.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          user_sign_up_by_email({
            realName: values.realName,
            nickname: values.nickname,
            email: values.email,
            password: values.password,
            timezone: guess_timezone(),
          })
            .then(() => {
              messageApi
                .success(
                  intl.formatMessage({ id: "pages.users.sign-up.instruction" })
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
        formKey="users.sign-up-by-email"
        request={async () => {
          return {
            realName: "",
            nickname: "",
            email: "",
            password: "",
            passwordConfirmation: "",
          };
        }}
        autoFocusFirstInput
      >
        <ProFormText
          name="realName"
          width="md"
          label={<FormattedMessage id="form.fields.real-name.label" />}
          rules={[
            { required: true },
            { min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH },
          ]}
        />
        <ProFormText
          name="email"
          width="md"
          label={<FormattedMessage id="form.fields.email.label" />}
          rules={[
            { required: true },
            { min: EMAIL_MIN_LENGTH, max: EMAIL_MAX_LENGTH },
          ]}
        />
        <ProFormText
          name="nickname"
          width="md"
          label={<FormattedMessage id="form.fields.nickname.label" />}
          rules={[
            { required: true },
            { min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH },
          ]}
        />
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
