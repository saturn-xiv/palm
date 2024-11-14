import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormTextArea } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { IError } from "../../api";
import { SIGN_IN_PATH } from "../../reducers/current-user";
import { create_leave_word } from "../../api/daffodil";

interface IFormValue {
  content: string;
}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  const navigate = useNavigate();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.leave-words.new.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          create_leave_word(values.content)
            .then(() => {
              messageApi
                .success(intl.formatMessage({ id: "flashes.succeed" }))
                .then(() => navigate(SIGN_IN_PATH));
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        formRef={formRef}
        formKey="leave-words.new"
        request={async () => {
          return {
            content: "",
          };
        }}
        autoFocusFirstInput
      >
        <ProFormTextArea
          width="md"
          name="content"
          label={<FormattedMessage id="form.fields.content.label" />}
          rules={[{ required: true }, { min: 31, max: 1023 }]}
        />
      </ProForm>
    </>
  );
};

export default Widget;
