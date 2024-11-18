import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Button, message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import { IError } from "../../../../api";
import {
  get_index_now_site_ownership_verifying,
  IIndexNowSiteOwnershipVerifying as IFormValue,
  set_index_now_site_ownership_verifying,
} from "../../../../api/daffodil";

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.index-now.ownership-verifying.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          set_index_now_site_ownership_verifying(values.key)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="admin.site.index-now-ownership-verifying"
        request={async () => {
          const res = await get_index_now_site_ownership_verifying();
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
                    .open("https://www.indexnow.org/documentation", "_blank")
                    ?.focus();
                }}
              >
                Verifying ownership via the key
              </Button>,
            ];
          },
        }}
        autoFocusFirstInput
      >
        <ProFormText
          name="key"
          width="md"
          label={<FormattedMessage id="form.fields.key.label" />}
          rules={[{ required: true }]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
