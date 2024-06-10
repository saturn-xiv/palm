import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { confirm_by_email } from "../../../api/users";
import { USERS_SIGN_IN_PATH } from "../../../Router";
import { IErrorMessage } from "../../../api/graphql";

interface IForm {
  user: string;
}

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();

  return (
    <Card
      title={<FormattedMessage id="users.confirm.by-email.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          confirm_by_email(values.user)
            .then(() => {
              messageApi.open({
                type: "success",
                content: intl.formatMessage({
                  id: "users.confirm.by-email.succeed",
                }),
                onClose: () => {
                  navigate(USERS_SIGN_IN_PATH);
                },
              });
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
      >
        <ProFormText
          width="md"
          name="user"
          label={<FormattedMessage id="form.fields.account.label" />}
          rules={[{ required: true }]}
        />
      </ProForm>
    </Card>
  );
};
