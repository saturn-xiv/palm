import { ProForm, ProFormTextArea } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { create_leave_word } from "../../api/leave-words";
import { EDITOR_TEXTAREA, IErrorMessage } from "../../api/graphql";

interface IForm {
  content: string;
}

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();

  return (
    <Card title={<FormattedMessage id="leave-words.new.title" />} hoverable>
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          create_leave_word(values.content, EDITOR_TEXTAREA)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
      >
        <ProFormTextArea
          width="md"
          name="content"
          label={<FormattedMessage id="form.fields.content.label" />}
          rules={[{ required: true, min: 15, max: 511 }]}
        />
      </ProForm>
    </Card>
  );
};
