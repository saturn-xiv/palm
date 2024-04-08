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
  delete_google_recaptcha,
  get_google_recaptcha,
  set_google_recaptcha,
} from "../../../../api/camelia";

interface IForm {
  site_key: string;
  secret: string;
}

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const formRef = useRef<ProFormInstance>();

  return (
    <Card
      title={<FormattedMessage id="settings.site.seo.recaptcha.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        formRef={formRef}
        onFinish={async (values) => {
          set_google_recaptcha(values.site_key, values.secret)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        request={async () => {
          const it = await get_google_recaptcha();
          return {
            site_key: it.siteKey,
            secret: it.secret,
          };
        }}
        submitter={{
          render: (_props, doms) => {
            return [
              ...doms,
              <DeleteButton
                key="delete"
                handleRefresh={() => {
                  formRef.current?.setFieldsValue({ site_key: "", secret: "" });
                }}
              />,
            ];
          },
        }}
      >
        <ProFormText
          width="md"
          name="site_key"
          label={
            <FormattedMessage id="settings.site.seo.recaptcha.fields.site-key.label" />
          }
          rules={[{ required: true }]}
        />
        <ProFormText
          width="md"
          name="secret"
          label={<FormattedMessage id="form.fields.secret.label" />}
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
        delete_google_recaptcha()
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
