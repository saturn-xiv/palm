import {
  ModalForm,
  ProForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { FormattedMessage, useIntl } from "react-intl";
import { Button, Form, Tooltip } from "antd";

import { SisternodeOutlined } from "@ant-design/icons";

import { IError } from "../../../api";
import { append_category } from "../../../api/daffodil";
import { IProps } from "./Create";

interface IFormValue {
  code: string;
  near?: number;
}

const Widget = ({ nodes, messageApi, handleRefresh }: IProps) => {
  const intl = useIntl();
  const [form] = Form.useForm<IFormValue>();
  const title = `buttons.append`;
  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id={title} />}
      trigger={
        <Tooltip title={<FormattedMessage id={title} />}>
          <Button icon={<SisternodeOutlined />} shape="circle" size="small" />
        </Tooltip>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        if (values.near) {
          append_category(values.near, values.code)
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
        name="near"
        label={<FormattedMessage id="form.fields.right-of.label" />}
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
