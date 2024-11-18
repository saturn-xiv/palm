import type { ProFormInstance } from "@ant-design/pro-components";
import {
  ProForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { Button, message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../../../api";
import { get_site_smtp, set_site_smtp } from "../../../../api/daffodil";
import {
  PASSWORD_MAX_LENGTH,
  PASSWORD_MIN_LENGTH,
} from "../../../../components";

interface IFormValue {
  host: string;
  port: number;
  account: string;
  password: string;
  passwordConfirmation: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.smtp.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_site_smtp(
            values.host,
            values.port,
            values.account,
            values.password
          )
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="admin.site.smtp"
        request={async () => {
          const res = await get_site_smtp();
          return {
            passwordConfirmation: "",
            password: "",
            ...res,
          };
        }}
        submitter={{
          render: (_, doms) => {
            return [
              ...doms,
              <Button
                key="gmail"
                type="link"
                onClick={(e) => {
                  e.preventDefault();
                  window
                    .open(
                      "https://support.google.com/a/answer/176600?hl=en",
                      "_blank"
                    )
                    ?.focus();
                }}
              >
                GMail
              </Button>,
              <Button
                key="qq.email"
                type="link"
                onClick={(e) => {
                  e.preventDefault();
                  window
                    .open("https://service.mail.qq.com/detail/0/427", "_blank")
                    ?.focus();
                }}
              >
                QQ邮箱
              </Button>,
            ];
          },
        }}
        autoFocusFirstInput
      >
        <ProFormText
          name="host"
          width="md"
          label={<FormattedMessage id="form.fields.host.label" />}
          rules={[{ required: true }]}
        />
        <ProFormSelect
          width="md"
          name="port"
          label={<FormattedMessage id="form.fields.port.label" />}
          options={[25, 465, 587].map((x) => {
            return {
              label: x,
              value: x,
            };
          })}
          rules={[{ required: true }]}
        />
        <ProFormText
          name="account"
          width="md"
          label={<FormattedMessage id="form.fields.account.label" />}
          rules={[{ required: true }]}
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
