import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { confirm_by_email } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { USERS_SIGN_IN_PATH } from "../../../Router";

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
              messageApi.success(
                intl.formatMessage({ id: "users.confirm.by-email.succeed" })
              );
              navigate(USERS_SIGN_IN_PATH);
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
      >
        <ProFormText
          width="sm"
          name="user"
          label={<FormattedMessage id="form.fields.account.label" />}
          rules={[{ required: true }]}
        />
      </ProForm>
    </Card>
  );
};
