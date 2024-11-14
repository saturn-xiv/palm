import type { ProFormInstance } from "@ant-design/pro-components";
import {
  ProForm,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { IError } from "../../api";
import { SIGN_IN_PATH } from "../../reducers/current-user";
import { cancel_my_email_account } from "../../api/daffodil";
import { PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH } from "../../components";

interface IFormValue {
  reason: string;
  password: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  const navigate = useNavigate();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.cancel.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          cancel_my_email_account(values.password, values.reason)
            .then(() => {
              messageApi
                .success(
                  intl.formatMessage({ id: "pages.users.cancel.succeed" })
                )
                .then(() => navigate(SIGN_IN_PATH));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="user.cancel"
        request={async () => {
          return {
            password: "",
            reason: "",
          };
        }}
        autoFocusFirstInput
      >
        <ProFormText.Password
          name="password"
          width="md"
          label={<FormattedMessage id="form.fields.current-password.label" />}
          rules={[
            { required: true },
            { min: PASSWORD_MIN_LENGTH, max: PASSWORD_MAX_LENGTH },
          ]}
        />
        <ProFormTextArea
          width="md"
          name="reason"
          label={<FormattedMessage id="form.fields.reason.label" />}
          rules={[{ required: true }, { min: 1, max: 1023 }]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
