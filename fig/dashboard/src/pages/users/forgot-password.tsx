import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { IError } from "../../api";
import { send_forgot_password_email_for_user } from "../../api/daffodil";
import { SIGN_IN_PATH } from "../../reducers/current-user";
import { NAME_MAX_LENGTH, NAME_MIN_LENGTH } from "../../components";

interface IFormValue {
  user: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  const navigate = useNavigate();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.forgot-password.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          send_forgot_password_email_for_user(values.user)
            .then(() => {
              messageApi
                .success(
                  intl.formatMessage({
                    id: "pages.users.forgot-password.instruction",
                  })
                )
                .then(() => navigate(SIGN_IN_PATH));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="users.forgot-password"
        request={async () => {
          return {
            user: "",
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
      </ProForm>
    </>
  );
};

export default Widget;
