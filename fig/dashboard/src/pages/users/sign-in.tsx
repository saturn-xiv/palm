import type { ProFormInstance } from "@ant-design/pro-components";
import {
  ProForm,
  ProFormSwitch,
  ProFormText,
} from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { IError } from "../../api";
import { user_sign_in_by_email } from "../../api/daffodil";
import { useAppDispatch } from "../../hooks";
import { PERSONAL_PATH, signIn } from "../../reducers/current-user";
import {
  NAME_MAX_LENGTH,
  NAME_MIN_LENGTH,
  PASSWORD_MAX_LENGTH,
  PASSWORD_MIN_LENGTH,
} from "../../components";

interface IFormValue {
  user: string;
  password: string;
  rememberMe: boolean;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  const navigate = useNavigate();
  const dispatch = useAppDispatch();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.sign-in.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          user_sign_in_by_email(values.user, values.password)
            .then((res) => {
              messageApi
                .open({
                  type: "success",
                  content: intl.formatMessage({
                    id: "pages.users.sign-in.succeed",
                  }),
                  duration: 1,
                })
                .then(() => {
                  dispatch(signIn(res));
                  navigate(PERSONAL_PATH);
                });
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="users.sign-in-by-email"
        request={async () => {
          return {
            user: "",
            password: "",
            rememberMe: false,
          };
        }}
        autoFocusFirstInput
      >
        <ProFormText
          name="user"
          width="md"
          label={
            <FormattedMessage id="pages.users.sign-in.form.email-or-nickname.label" />
          }
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
        <ProFormSwitch
          colProps={{
            span: 4,
          }}
          label={
            <FormattedMessage id="pages.users.sign-in.form.remember-me.label" />
          }
          name="rememberMe"
        />
      </ProForm>
    </>
  );
};

export default Widget;
