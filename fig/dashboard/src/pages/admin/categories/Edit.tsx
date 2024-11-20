import { ModalForm, ProForm, ProFormText } from "@ant-design/pro-components";
import { FormattedMessage, useIntl } from "react-intl";
import { Button, Form, Tooltip } from "antd";
import type { MessageInstance } from "antd/es/message/interface";
import { EditOutlined } from "@ant-design/icons";

import { IError } from "../../../api";
import { ICategory, update_category } from "../../../api/daffodil";

export interface IProps {
  handleRefresh: () => void;
  messageApi: MessageInstance;
  item: ICategory;
}
interface IFormValue {
  code: string;
}

const Widget = ({ item, messageApi, handleRefresh }: IProps) => {
  const intl = useIntl();
  const [form] = Form.useForm<IFormValue>();
  const title = `buttons.edit`;
  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id={title} />}
      trigger={
        <Tooltip title={<FormattedMessage id={title} />}>
          <Button icon={<EditOutlined />} shape="circle" size="small" />
        </Tooltip>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        update_category(item.id, values.code)
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
      }}
      request={async () => {
        return { code: item.code };
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
