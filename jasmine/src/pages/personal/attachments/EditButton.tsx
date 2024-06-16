import { EditOutlined } from "@ant-design/icons";
import { ModalForm, ProFormText } from "@ant-design/pro-components";
import { Form, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { IAttachment, update_attachment } from "../../../api/attachments";
import { IErrorMessage } from "../../../api/graphql";

interface IProps {
  item: IAttachment;
  handleRefresh: () => void;
}

interface IForm {
  title: string;
}
const Widget = ({ item, handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const [form] = Form.useForm<IForm>();
  return (
    <ModalForm<IForm>
      title={
        <FormattedMessage
          id="personal.attachments.edit.title"
          values={{ id: item.id }}
        />
      }
      trigger={<EditOutlined />}
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
        onCancel: () => {},
      }}
      submitTimeout={2000}
      onFinish={async (values) => {
        update_attachment(item.id, values.title)
          .then(() => {
            handleRefresh();
            messageApi.info({
              type: "success",
              content: intl.formatMessage({ id: "flashes.succeed" }),
              duration: 2,
            });
          })
          .catch((reason: IErrorMessage[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
          });
        return true;
      }}
    >
      {contextHolder}
      <ProFormText
        name="title"
        width="md"
        label={<FormattedMessage id="form.fields.title.label" />}
        initialValue={item.title}
      />
    </ModalForm>
  );
};

export default Widget;
