import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../../../api";
import { get_site_author, set_site_author } from "../../../../api/daffodil";
import { IAuthor as IFormValue } from "../../../../reducers/site";

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.author.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_site_author(values.name, values.email)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="admin.site.author"
        request={async () => {
          const res = await get_site_author();
          return res;
        }}
        autoFocusFirstInput
      >
        <ProFormText
          name="name"
          width="md"
          label={<FormattedMessage id="form.fields.username.label" />}
          rules={[{ required: true }]}
        />
        <ProFormText
          name="email"
          width="md"
          label={<FormattedMessage id="form.fields.email.label" />}
          rules={[{ required: true }, { type: "email" }]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
