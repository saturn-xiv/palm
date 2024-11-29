import { PlusOutlined } from "@ant-design/icons";
import {
  ModalForm,
  ProForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { Button, Form } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";
import { useState } from "react";

import HtmlEditor, { EDITOR } from "../../../components/WangEditor";
import { create_page, templates } from "../../../api/cms";
import {
  SLUG_MAX_LENGTH,
  SLUG_MIN_LENGTH,
  TITLE_MAX_LENGTH,
  TITLE_MIN_LENGTH,
} from "../../../components";
import { available_languages } from "../../../i18n";
import { IError } from "../../../api";

interface IFormValue {
  lang: string;
  title: string;
  slug: string;
  template: string;
}
interface IProps {
  messageApi: MessageInstance;
  handleReload: () => void;
}
const Widget = ({ messageApi, handleReload }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const [body, setBody] = useState("");
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id="pages.cms.pages.new.title" />}
      trigger={
        <Button icon={<PlusOutlined />} type="primary">
          <FormattedMessage id="buttons.new" />
        </Button>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        const ok = await create_page(
          values.lang,
          values.slug,
          values.title,
          values.template.toUpperCase(),
          body,
          EDITOR
        )
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
      <ProFormSelect
        width="xs"
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
        name="slug"
        label={<FormattedMessage id="form.fields.slug.label" />}
        rules={[
          { required: true },
          { min: SLUG_MIN_LENGTH, max: SLUG_MAX_LENGTH },
        ]}
      />
      <ProFormText
        name="title"
        label={<FormattedMessage id="form.fields.title.label" />}
        rules={[
          { required: true },
          { min: TITLE_MIN_LENGTH, max: TITLE_MAX_LENGTH },
        ]}
      />
      <ProFormSelect
        request={async () =>
          templates.map((x) => {
            return {
              value: x,
              label: intl.formatMessage({
                id: `pages.cms.page-templates.${x}.label`,
              }),
            };
          })
        }
        width="xs"
        name="template"
        label={<FormattedMessage id="form.fields.template.label" />}
        rules={[{ required: true }]}
      />
      <ProForm.Group>
        <HtmlEditor html="" handleChange={setBody} />
      </ProForm.Group>
    </ModalForm>
  );
};

export default Widget;
