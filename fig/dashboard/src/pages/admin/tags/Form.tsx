import { ModalForm, ProForm, ProFormText } from "@ant-design/pro-components";
import { FormattedMessage, useIntl } from "react-intl";
import { Button, Form, Tooltip } from "antd";
import type { MessageInstance } from "antd/es/message/interface";
import { PlusOutlined, EditOutlined } from "@ant-design/icons";

import { IError } from "../../../api";
import { create_tag, ITag, update_tag } from "../../../api/daffodil";

interface IProps {
  handleRefresh: () => void;
  messageApi: MessageInstance;
  item?: ITag;
}
interface IFormValue {
  code: string;
}

const Widget = ({ item, messageApi, handleRefresh }: IProps) => {
  const intl = useIntl();
  const [form] = Form.useForm<IFormValue>();
  const title = `buttons.${item ? "edit" : "new"}`;
  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id={title} />}
      trigger={
        <Tooltip title={<FormattedMessage id={title} />}>
          <Button
            icon={item ? <EditOutlined /> : <PlusOutlined />}
            shape="circle"
            size="small"
          />
        </Tooltip>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        if (item === undefined) {
          create_tag(values.code)
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
        } else {
          update_tag(item.id, values.code)
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
        return { code: item ? item.code : "" };
      }}
    >
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
