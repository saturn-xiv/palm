import {
  ModalForm,
  ProFormSelect,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Form, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { EditOutlined, PlusOutlined } from "@ant-design/icons";

import { useAppSelector } from "../../../hooks";
import { siteInfo as selectSiteInfo } from "../../../reducers/site-info";
import { IErrorMessage } from "../../../api/graphql";
import { save as save_locale } from "../../../api/locales";

interface IForm {
  lang: string;
  code: string;
  message: string;
}

interface IProps {
  item: IForm;
  edit?: boolean;
  handleRefresh: () => void;
}

const Widget = ({ item, edit, handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const site_info = useAppSelector(selectSiteInfo);
  const [form] = Form.useForm<IForm>();
  return (
    <ModalForm<IForm>
      title={<FormattedMessage id="buttons.edit" />}
      trigger={edit ? <EditOutlined /> : <PlusOutlined />}
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
        onCancel: () => {},
      }}
      submitTimeout={2000}
      onFinish={async (values) => {
        save_locale(values.lang, values.code, values.message)
          .then(() => {
            messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            handleRefresh();
          })
          .catch((reason: IErrorMessage[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
          });
        return true;
      }}
      initialValues={{
        lang: item.lang,
        code: item.code,
        message: item.message,
      }}
    >
      {contextHolder}
      <ProFormSelect
        options={site_info.languages.map((x: string) => {
          return {
            value: x,
            label: intl.formatMessage({ id: `languages.${x}` }),
          };
        })}
        width="md"
        cacheForSwr
        name="lang"
        label={<FormattedMessage id="form.fields.language.label" />}
      />
      <ProFormText
        width="md"
        name="code"
        label={<FormattedMessage id="form.fields.code.label" />}
        rules={[
          {
            required: true,
          },
        ]}
      />
      <ProFormTextArea
        width="md"
        name="message"
        label={<FormattedMessage id="form.fields.message.label" />}
        rules={[{ required: true }]}
      />
    </ModalForm>
  );
};

export default Widget;
