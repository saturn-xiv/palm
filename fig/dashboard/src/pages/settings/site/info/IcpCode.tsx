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
  delete_site_icp_code,
  fetch_layout,
  set_site_icp_code,
} from "../../../../api/camelia";
import { get as get_locale } from "../../../../locales";

interface IProps {
  handleRefresh: () => void;
}
interface IForm {
  code: string;
}

const Widget = ({ handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const formRef = useRef<ProFormInstance>();

  return (
    <Card
      title={<FormattedMessage id="settings.site.info.icp-code.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        formRef={formRef}
        onFinish={async (values) => {
          set_site_icp_code(values.code)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
              handleRefresh();
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        request={async () => {
          const it = await fetch_layout(get_locale());
          return {
            code: it.siteInfo.icpCode?.code || "",
          };
        }}
        submitter={{
          render: (_props, doms) => {
            return [
              ...doms,
              <DeleteButton
                key="delete"
                handleRefresh={() => {
                  formRef.current?.setFieldsValue({ code: "" });
                }}
              />,
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
      </ProForm>
    </Card>
  );
};

export default Widget;

const DeleteButton = ({ handleRefresh }: IProps) => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  return (
    <Popconfirm
      title={<FormattedMessage id="flashes.are-you-sure" />}
      onConfirm={() => {
        delete_site_icp_code()
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
