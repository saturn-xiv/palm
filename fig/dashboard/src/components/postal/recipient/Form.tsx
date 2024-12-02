import { PlusOutlined, EditOutlined } from "@ant-design/icons";
import { ModalForm, ProFormText } from "@ant-design/pro-components";
import { Button, Form } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";

import {
  IPostalRecipient,
  IPostalRecipientFormValue,
} from "../../../api/daffodil";
import { IError } from "../../../api";
import {
  EMAIL_MAX_LENGTH,
  EMAIL_MIN_LENGTH,
  NAME_MAX_LENGTH,
  NAME_MIN_LENGTH,
} from "../..";

interface IProps {
  item?: IPostalRecipient;
  messageApi: MessageInstance;
  handleSave: (it: IPostalRecipientFormValue) => Promise<void>;
  handleReload: () => void;
  title: string;
}

const Widget = ({
  title,
  item,
  messageApi,
  handleSave,
  handleReload,
}: IProps) => {
  const [form] = Form.useForm<IPostalRecipientFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IPostalRecipientFormValue>
      title={title}
      trigger={
        <Button
          icon={item ? <EditOutlined /> : <PlusOutlined />}
          size="small"
          type="dashed"
        />
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        return {
          name: item?.name || "",
          email: item?.email,
          wechat: item?.wechat,
          phone: item?.phone,
          fax: item?.fax,
          whatsapp: item?.whatsapp,
        };
      }}
      onFinish={async (values) => {
        const ok = await handleSave(values)
          .then(async () => {
            await messageApi
              .success(intl.formatMessage({ id: "flashes.succeed" }))
              .then(() => {
                handleReload();
              });
            return true;
          })
          .catch((reason: IError[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
            return false;
          });
        return ok;
      }}
    >
      <ProFormText
        name="name"
        label={<FormattedMessage id="form.fields.username.label" />}
        rules={[
          { required: true },
          { min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH },
        ]}
      />
      <ProFormText
        name="phone"
        label={<FormattedMessage id="form.fields.phone.label" />}
        rules={[{ min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH }]}
      />
      <ProFormText
        name="fax"
        label={<FormattedMessage id="form.fields.fax.label" />}
        rules={[{ min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH }]}
      />
      <ProFormText
        name="email"
        label={<FormattedMessage id="form.fields.email.label" />}
        rules={[{ min: EMAIL_MIN_LENGTH, max: EMAIL_MAX_LENGTH }]}
      />
      <ProFormText
        name="wechat"
        label={<FormattedMessage id="form.fields.wechat.label" />}
        rules={[{ min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH }]}
      />
      <ProFormText
        name="whatsapp"
        label="WhatsApp"
        rules={[{ min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH }]}
      />
    </ModalForm>
  );
};

export default Widget;
