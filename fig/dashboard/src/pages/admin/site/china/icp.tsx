import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { message, Button, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../../../api";
import { get_site_cn_icp, set_site_cn_icp } from "../../../../api/daffodil";
import { ICnIcp as IFormValue } from "../../../../reducers/site";

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.cn-icp.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_site_cn_icp(values.code)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="admin.site.cn-icp"
        request={async () => {
          const res = await get_site_cn_icp();
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
                  window.open("https://beian.miit.gov.cn", "_blank")?.focus();
                }}
              >
                <FormattedMessage id="pages.admin.site.cn-icp.homage.label" />
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
      </ProForm>
    </>
  );
};

export default Widget;
