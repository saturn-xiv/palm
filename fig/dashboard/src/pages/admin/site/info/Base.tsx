import type { ProFormInstance } from "@ant-design/pro-components";
import {
  ProForm,
  ProFormSelect,
  ProFormTextArea,
  ProFormText,
} from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../../../api";
import {
  get_site_info_by_lang,
  set_site_base_info,
} from "../../../../api/daffodil";
import { available_languages } from "../../../../i18n";

interface IFormValue {
  lang: string;
  title: string;
  subhead: string;
  description: string;
  copyright: string;
}

interface IProps {
  lang: string;
}

const Widget = ({ lang }: IProps) => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.base.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_site_base_info(values.lang, {
            title: values.title,
            subhead: values.subhead,
            description: values.description,
            copyright: values.copyright,
          })
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="admin.site.base-info"
        request={async () => {
          const res = await get_site_info_by_lang(lang);
          return {
            lang,
            title: res.title,
            subhead: res.subhead,
            description: res.description,
            copyright: res.copyright,
          };
        }}
        autoFocusFirstInput
      >
        <ProFormSelect
          name="lang"
          width="md"
          label={<FormattedMessage id="form.fields.lang.label" />}
          onChange={(v: string) => {
            get_site_info_by_lang(v).then((res) => {
              formRef.current?.setFieldsValue(res);
            });
          }}
          options={available_languages.map((x) => {
            return {
              label: intl.formatMessage({ id: `languages.${x}` }),
              value: x,
            };
          })}
          rules={[{ required: true }]}
        />
        <ProFormText
          name="title"
          width="md"
          label={<FormattedMessage id="form.fields.title.label" />}
          rules={[{ required: true }]}
        />
        <ProFormText
          name="subhead"
          width="md"
          label={
            <FormattedMessage id="pages.admin.site.base.form.subhead.label" />
          }
          rules={[{ required: true }]}
        />
        <ProFormTextArea
          name="description"
          width="md"
          label={<FormattedMessage id="form.fields.description.label" />}
          rules={[{ required: true }]}
        />
        <ProFormText
          name="copyright"
          width="md"
          label={
            <FormattedMessage id="pages.admin.site.base.form.copyright.label" />
          }
          rules={[{ required: true }]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
