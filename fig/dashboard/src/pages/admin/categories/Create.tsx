import {
  ModalForm,
  ProForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { FormattedMessage, useIntl } from "react-intl";
import { Button, Form, Tooltip } from "antd";
import type { MessageInstance } from "antd/es/message/interface";
import { SubnodeOutlined } from "@ant-design/icons";

import { IError } from "../../../api";
import { create_category, ICategory } from "../../../api/daffodil";

export interface IProps {
  handleRefresh: () => void;
  messageApi: MessageInstance;
  nodes: ICategory[];
}
interface IFormValue {
  code: string;
  parent?: number;
}

const Widget = ({ nodes, messageApi, handleRefresh }: IProps) => {
  const intl = useIntl();
  const [form] = Form.useForm<IFormValue>();
  const title = `buttons.new`;
  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id={title} />}
      trigger={
        <Tooltip title={<FormattedMessage id={title} />}>
          <Button icon={<SubnodeOutlined />} shape="circle" size="small" />
        </Tooltip>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        if (values.parent) {
          create_category(values.parent, values.code)
            .then(() => {
              messageApi
                .success(intl.formatMessage({ id: "flashes.succeed" }))
                .then(() => {
                  handleRefresh();
                });
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }
        return true;
      }}
      request={async () => {
        return { code: "" };
      }}
    >
      <ProFormSelect
        width="md"
        name="parent"
        label={<FormattedMessage id="form.fields.parent.label" />}
        options={nodes.map((x) => {
          return {
            label: x.code,
            value: x.id,
          };
        })}
        rules={[{ required: true }]}
      />
      <ProForm.Group>
        <ProFormText
          width="md"
          name="code"
          label={<FormattedMessage id="form.fields.code.label" />}
        />
      </ProForm.Group>
    </ModalForm>
  );
};

export default Widget;
