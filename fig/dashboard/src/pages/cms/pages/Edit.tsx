import { EditOutlined } from "@ant-design/icons";
import { ModalForm, ProForm, ProFormText } from "@ant-design/pro-components";
import { Button, Form } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";
import { useState } from "react";

import HtmlEditor from "../../../components/WangEditor";
import { IPage, update_page } from "../../../api/cms";
import {
  SLUG_MAX_LENGTH,
  SLUG_MIN_LENGTH,
  TITLE_MAX_LENGTH,
  TITLE_MIN_LENGTH,
} from "../../../components";
import { IError } from "../../../api";

interface IFormValue {
  title: string;
  slug: string;
}
interface IProps {
  item: IPage;
  messageApi: MessageInstance;
  handleReload: () => void;
}
const Widget = ({ item, messageApi, handleReload }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const [body, setBody] = useState(item.body);
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={item.title}
      trigger={
        <Button icon={<EditOutlined />} size="small" type="dashed">
          <FormattedMessage id="buttons.edit" />
        </Button>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        return {
          title: item.title,
          slug: item.slug,
        };
      }}
      submitter={{
        render: (_props, defaultDoms) => {
          return [
            ...defaultDoms,
            <Button
              key="delete"
              danger
              onClick={() => {
                // TODO
                console.log("delete", item.id);
              }}
            >
              <FormattedMessage id="buttons.delete" />
            </Button>,
          ];
        },
      }}
      onFinish={async (values) => {
        const ok = await update_page(item.id, values.slug, values.title, body)
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
      <ProForm.Group>
        <HtmlEditor html={body} handleChange={setBody} />
      </ProForm.Group>
    </ModalForm>
  );
};

export default Widget;
