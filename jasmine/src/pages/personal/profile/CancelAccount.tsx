import { ProForm, ProFormTextArea } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { cancel_account } from "../../../api/users";
import { IErrorMessage } from "../../../api/graphql";
import { USERS_SIGN_IN_PATH } from "../../../Router";
import {
  remove as remove_token,
  signOut,
} from "../../../reducers/current-user";
import { useAppDispatch } from "../../../hooks";

interface IForm {
  reason: string;
}

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  return (
    <Card
      title={<FormattedMessage id="personal.profile.cancel-account.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          cancel_account(values.reason)
            .then(() => {
              remove_token();
              messageApi.info({
                type: "success",
                content: intl.formatMessage({
                  id: "personal.profile.cancel-account.succeed",
                }),
                onClose: () => {
                  dispatch(signOut());
                  navigate(USERS_SIGN_IN_PATH);
                },
                duration: 1,
              });
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
      >
        <ProFormTextArea
          width="md"
          name="reason"
          label={<FormattedMessage id="form.fields.reason.label" />}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;
