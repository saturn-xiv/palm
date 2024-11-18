import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Button, message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../../../api";
import {
  get_google_site_ownership_verifying,
  IGoogleSiteOwnershipVerifying as IFormValue,
  set_google_site_ownership_verifying,
} from "../../../../api/daffodil";

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.google.ownership-verifying.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_google_site_ownership_verifying(values.code)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="admin.site.google-ownership-verifying"
        request={async () => {
          const res = await get_google_site_ownership_verifying();
          return res;
        }}
        submitter={{
          render: (_, doms) => {
            return [
              ...doms,
              <Button
                key="help"
                type="link"
                onClick={(e) => {
                  e.preventDefault();
                  window
                    .open(
                      "https://support.google.com/webmasters/answer/9008080?hl=en#meta_tag_verification",
                      "_blank"
                    )
                    ?.focus();
                }}
              >
                HTML tag
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
