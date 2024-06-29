import { useRef } from "react";
import {
  ProForm,
  ProFormInstance,
  ProFormText,
} from "@ant-design/pro-components";
import { Button, Card, Popconfirm, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { IErrorMessage } from "../../../../api/graphql";
import {
  delete_index_now_site_verification,
  get_index_now_site_verification,
  set_index_now_site_verification,
} from "../../../../api/site";

interface IForm {
  key: string;
}

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const formRef = useRef<ProFormInstance>();

  return (
    <Card
      title={<FormattedMessage id="settings.site.seo.index-now.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        formRef={formRef}
        onFinish={async (values) => {
          set_index_now_site_verification(values.key)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        request={async () => {
          const it = await get_index_now_site_verification();
          return {
            key: it.key,
          };
        }}
        submitter={{
          render: (_props, doms) => {
            return [
              ...doms,
              <DeleteButton
                key="delete"
                handleRefresh={() => {
                  formRef.current?.setFieldsValue({ key: "" });
                }}
              />,
            ];
          },
        }}
      >
        <ProFormText
          width="md"
          name="key"
          label={<FormattedMessage id="form.fields.key.label" />}
          rules={[{ required: true }]}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;

const DeleteButton = ({ handleRefresh }: { handleRefresh: () => void }) => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  return (
    <Popconfirm
      title={<FormattedMessage id="flashes.are-you-sure" />}
      onConfirm={() => {
        delete_index_now_site_verification()
          .then(() => {
            messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            handleRefresh();
          })
          .catch((reason: IErrorMessage[]) => {
            message.error(reason.map((x) => x.message).join("\n"));
          });
      }}
      okText={<FormattedMessage id="buttons.ok" />}
      cancelText={<FormattedMessage id="buttons.cancel" />}
    >
      {contextHolder}
      <Button type="dashed">
        <FormattedMessage id="buttons.delete" />
      </Button>
    </Popconfirm>
  );
};
