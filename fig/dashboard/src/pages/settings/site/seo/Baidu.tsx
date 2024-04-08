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
  delete_baidu_site_verification,
  get_baidu_site_verification,
  ping_baidu,
  set_baidu_site_verification,
} from "../../../../api/camelia";
import { home_url } from "../../../../utils";

interface IForm {
  code: string;
  content: string;
}

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const formRef = useRef<ProFormInstance>();

  return (
    <Card
      title={<FormattedMessage id="settings.site.seo.baidu.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        formRef={formRef}
        onFinish={async (values) => {
          set_baidu_site_verification(values.code, values.content)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        request={async () => {
          const it = await get_baidu_site_verification();
          return {
            code: it.code,
            content: it.content,
          };
        }}
        submitter={{
          render: (_props, doms) => {
            return [
              ...doms,
              <DeleteButton
                key="delete"
                handleRefresh={() => {
                  formRef.current?.setFieldsValue({ code: "", content: "" });
                }}
              />,
              <PingButton key="ping" />,
            ];
          },
        }}
      >
        <ProFormText
          width="md"
          name="code"
          label={<FormattedMessage id="form.fields.code.label" />}
          rules={[{ required: true }]}
        />
        <ProFormText
          width="md"
          name="content"
          label={<FormattedMessage id="form.fields.content.label" />}
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
        delete_baidu_site_verification()
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

const PingButton = () => {
  const intl = useIntl();
  const home = home_url();
  const [messageApi, contextHolder] = message.useMessage();
  return (
    <Popconfirm
      title={<FormattedMessage id="flashes.are-you-sure" />}
      onConfirm={() => {
        ping_baidu(home)
          .then(() => {
            messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
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
        <FormattedMessage id="buttons.ping" />
      </Button>
    </Popconfirm>
  );
};
