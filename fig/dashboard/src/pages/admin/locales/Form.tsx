import {
  ModalForm,
  ProForm,
  ProFormTextArea,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { FormattedMessage, useIntl } from "react-intl";
import { Button, Form, Tooltip } from "antd";
import type { MessageInstance } from "antd/es/message/interface";
import { PlusOutlined, EditOutlined } from "@ant-design/icons";

import { set_locale } from "../../../api/daffodil";
import { IError } from "../../../api";
import { available_languages, DEFAULT_LANGUAGE } from "../../../i18n";

interface IProps {
  handleRefresh: () => void;
  messageApi: MessageInstance;
  item?: IValue;
}
interface IValue {
  lang: string;
  code: string;
  message: string;
}

const Widget = ({ item, messageApi, handleRefresh }: IProps) => {
  const intl = useIntl();
  const [form] = Form.useForm<IValue>();
  const title = `buttons.${item ? "edit" : "new"}`;
  return (
    <ModalForm<IValue>
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
        set_locale(values.lang, values.code, values.message)
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
        return true;
      }}
      request={async () => {
        return (
          item || {
            code: "",
            lang: DEFAULT_LANGUAGE,
            message: "",
          }
        );
      }}
    >
      <ProForm.Group>
        <ProFormSelect
          name="lang"
          label={<FormattedMessage id="form.fields.lang.label" />}
          options={available_languages.map((x) => {
            return {
              label: intl.formatMessage({ id: `languages.${x}` }),
              value: x,
            };
          })}
          rules={[{ required: true }]}
        />
        <ProFormText
          width="md"
          name="code"
          label={<FormattedMessage id="form.fields.code.label" />}
        />
      </ProForm.Group>
      <ProForm.Group>
        <ProFormTextArea
          width="md"
          name="message"
          label={<FormattedMessage id="form.fields.message.label" />}
          rules={[{ required: true }, { min: 1, max: 2048 }]}
        />
      </ProForm.Group>
    </ModalForm>
  );
};

export default Widget;
