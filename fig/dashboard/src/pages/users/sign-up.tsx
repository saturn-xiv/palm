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

interface IFormValue {
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
        />
        <ProFormText
          name="email"
          width="md"
          label={<FormattedMessage id="form.fields.email.label" />}
        />
        <ProFormText
          name="nickname"
          width="md"
          label={<FormattedMessage id="form.fields.nickname.label" />}
        />
        <ProFormText.Password
          name="password"
          width="md"
          label={<FormattedMessage id="form.fields.password.label" />}
        />
        <ProFormText.Password
          name="passwordConfirmation"
          width="md"
          label={
            <FormattedMessage id="form.fields.password-confirmation.label" />
          }
        />
      </ProForm>
    </>
  );
};

export default Widget;
