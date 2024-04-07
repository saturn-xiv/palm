import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { sign_in_by_email } from "../../api/camelia";
import { useAppDispatch } from "../../hooks";
import { signIn } from "../../reducers/current-user";
import { IErrorMessage } from "../../api/graphql";
import { SELF_PATH } from "../../Router";

interface IForm {
  user: string;
  password: string;
}

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  return (
    <Card title={<FormattedMessage id="users.sign-in.title" />} hoverable>
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          sign_in_by_email(values.user, values.password)
            .then((res) => {
              messageApi.success(
                intl.formatMessage({ id: "users.sign-in.succeed" })
              );
              dispatch(signIn(res));
              navigate(SELF_PATH);
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
        <ProFormText.Password
          width="sm"
          name="password"
          label={<FormattedMessage id="form.fields.password.label" />}
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
