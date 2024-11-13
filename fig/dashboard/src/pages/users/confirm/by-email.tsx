import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { IError } from "../../../api";
import { send_confirm_email_for_user } from "../../../api/daffodil";
import { SIGN_IN_PATH } from "../../../reducers/current-user";

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
        <FormattedMessage id="pages.users.confirm.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          send_confirm_email_for_user(values.user)
            .then(() => {
              messageApi
                .success(
                  intl.formatMessage({ id: "pages.users.confirm.instruction" })
                )
                .then(() => navigate(SIGN_IN_PATH));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="users.confirm-by-email"
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
        />
      </ProForm>
    </>
  );
};

export default Widget;
