import type { ProFormInstance } from "@ant-design/pro-components";
import {
  ProForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../api";
import {
  get_email_user_profile,
  set_email_user_profile,
} from "../../api/daffodil";
import { NAME_MAX_LENGTH, NAME_MIN_LENGTH } from "../../components";
import { available_languages, set as set_locale } from "../../i18n";
import { timezones } from "../../utils";

interface IFormValue {
  realName: string;
  timezone: string;
  lang: string;
  email: string;
  nickname: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();

  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.update-profile.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_email_user_profile(values.realName, values.lang, values.timezone)
            .then(() => {
              messageApi
                .success(intl.formatMessage({ id: "flashes.succeed" }))
                .then(() => {
                  set_locale(values.lang, true);
                });
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="users.update-profile"
        request={async () => {
          const it = await get_email_user_profile();
          return it;
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
          name="nickname"
          width="md"
          label={<FormattedMessage id="form.fields.nickname.label" />}
          rules={[{ required: true }]}
          disabled
        />
        <ProFormText
          name="email"
          width="md"
          label={<FormattedMessage id="form.fields.email.label" />}
          rules={[{ required: true }]}
          disabled
        />
        <ProFormSelect
          width="md"
          name="lang"
          label={<FormattedMessage id="form.fields.lang.label" />}
          options={available_languages.map((x) => {
            return {
              label: intl.formatMessage({ id: `languages.${x}` }),
              value: x,
            };
          })}
          rules={[{ required: true }]}
        />
        <ProFormSelect
          width="md"
          name="timezone"
          label={<FormattedMessage id="form.fields.timezone.label" />}
          options={timezones().map((x) => {
            return {
              label: x,
              value: x,
            };
          })}
          rules={[{ required: true }]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
