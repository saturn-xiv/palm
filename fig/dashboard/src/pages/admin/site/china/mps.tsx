import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Button, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../../../api";
import { get_site_cn_mps, set_site_cn_mps } from "../../../../api/daffodil";
import { ICnMps as IFormValue } from "../../../../reducers/site";

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.cn-mps.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_site_cn_mps(values.code, values.name)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="admin.site.cn-mps"
        request={async () => {
          const res = await get_site_cn_mps();
          return res;
        }}
        submitter={{
          render: (_, doms) => {
            return [
              ...doms,
              <Button
                type="link"
                onClick={(e) => {
                  e.preventDefault();
                  window.open("https://beian.mps.gov.cn", "_blank")?.focus();
                }}
              >
                <FormattedMessage id="pages.admin.site.cn-mps.homage.label" />
              </Button>,
            ];
          },
        }}
        autoFocusFirstInput
      >
        <ProFormText
          name="code"
          width="md"
          label={<FormattedMessage id="form.fields.code.label" />}
          rules={[{ required: true }]}
        />
        <ProFormText
          name="name"
          width="md"
          label={<FormattedMessage id="form.fields.name.label" />}
          rules={[{ required: true }]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
