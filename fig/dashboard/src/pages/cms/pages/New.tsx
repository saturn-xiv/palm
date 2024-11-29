import { PlusOutlined } from "@ant-design/icons";
import { ModalForm, ProForm, ProFormText } from "@ant-design/pro-components";
import { Button, Form } from "antd";
import { FormattedMessage } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";
import { useState } from "react";

import HtmlEditor from "../../../components/WangEditor";

interface IFormValue {
  title: string;
  slug: string;
}
interface IProps {
  messageApi: MessageInstance;
}
const Widget = ({ messageApi }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const [body, setBody] = useState("");

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
      submitTimeout={2000}
      onFinish={async (values) => {
        console.log(values, body);
        messageApi.success("提交成功");
        return true;
      }}
    >
      <ProFormText
        width="md"
        name="slug"
        label={<FormattedMessage id="form.fields.slug.label" />}
      />
      <ProFormText
        name="title"
        label={<FormattedMessage id="form.fields.title.label" />}
      />
      <ProForm.Group>
        <HtmlEditor html="" handleChange={setBody} />
      </ProForm.Group>
    </ModalForm>
  );
};

export default Widget;
