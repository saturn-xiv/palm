import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { change_password } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { USERS_SIGN_IN_PATH } from "../../../Router";
import { PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH } from "../../users/sign-up";

interface IForm {
  current_password: string;
  new_password: string;
  password_confirmation: string;
}

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();

  return (
    <Card
      title={<FormattedMessage id="personal.profile.change-password.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          if (values.new_password !== values.password_confirmation) {
            messageApi.error(
              intl.formatMessage({
                id: "form.fields.password-confirmation.error",
              })
            );
            return;
          }
          change_password(values.current_password, values.new_password)
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
        <ProFormText.Password
          width="md"
          name="current_password"
          label={<FormattedMessage id="form.fields.current-password.label" />}
          rules={[
            {
              required: true,
            },
          ]}
        />
        <ProFormText.Password
          width="md"
          name="new_password"
          label={<FormattedMessage id="form.fields.new-password.label" />}
          rules={[
            {
              required: true,
              min: PASSWORD_MIN_LENGTH,
              max: PASSWORD_MAX_LENGTH,
            },
          ]}
        />
        <ProFormText.Password
          width="md"
          name="password_confirmation"
          label={
            <FormattedMessage id="form.fields.password-confirmation.label" />
          }
          rules={[
            {
              required: true,
            },
          ]}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;
