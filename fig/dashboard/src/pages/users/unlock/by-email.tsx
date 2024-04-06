import { useEffect } from "react";
import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Row, Col, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { unlock_by_email } from "../../../api/camelia";
import { IErrorMessage } from "../../../api/graphql";
import { useAppDispatch } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import {
  USERS_SIGN_IN_PATH,
  USERS_UNLOCK_BY_EMAIL_PATH,
} from "../../../Router";

interface IForm {
  user: string;
}

export const Component = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  useEffect(() => {
    dispatch(set_pathname(USERS_UNLOCK_BY_EMAIL_PATH));
  }, [dispatch]);

  return (
    <Row>
      <Col sm={24} md={{ span: 8, offset: 8 }}>
        {contextHolder}
        <ProForm<IForm>
          onFinish={async (values) => {
            unlock_by_email(values.user)
              .then(() => {
                messageApi.success(
                  intl.formatMessage({ id: "users.unlock.by-email.succeed" })
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
      </Col>
    </Row>
  );
};
