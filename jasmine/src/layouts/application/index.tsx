import { Outlet } from "react-router-dom";
import {
  LoginOutlined,
  IdcardOutlined,
  MailOutlined,
  MessageOutlined,
  UserAddOutlined,
  UnlockOutlined,
} from "@ant-design/icons";
import { Layout, Menu, Row, Col } from "antd";
import { useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import Footer from "../Footer";
import {
  LEAVE_WORDS_NEW_PATH,
  USERS_CONFIRM_BY_EMAIL_PATH,
  USERS_FORGOT_PASSWORD_BY_EMAIL_PATH,
  USERS_SIGN_IN_PATH,
  USERS_SIGN_UP_PATH,
  USERS_UNLOCK_BY_EMAIL_PATH,
} from "../../Router";
import background_img from "../../assets/background/redwoods.jpg";

export const Component = () => {
  const navigate = useNavigate();
  const intl = useIntl();

  return (
    <Layout>
      <Layout.Content>
        <Row justify="center" style={{ height: "100vh" }}>
          <Col
            sm={{ span: 0 }}
            md={{ span: 12 }}
            style={{
              backgroundImage: `url(${background_img})`,
              backgroundRepeat: "no-repeat",
              backgroundSize: "cover",
              backgroundPosition: "center",
            }}
          ></Col>
          <Col sm={24} md={{ span: 12 }}>
            <Outlet />
            <Menu
              onClick={(it) => {
                navigate(it.key);
              }}
              mode="inline"
              items={[
                {
                  key: USERS_SIGN_IN_PATH,
                  label: intl.formatMessage({ id: "users.sign-in.title" }),
                  icon: <LoginOutlined />,
                },
                {
                  key: USERS_SIGN_UP_PATH,
                  label: intl.formatMessage({ id: "users.sign-up.title" }),
                  icon: <UserAddOutlined />,
                },
                {
                  key: USERS_CONFIRM_BY_EMAIL_PATH,
                  label: intl.formatMessage({
                    id: "users.confirm.by-email.title",
                  }),
                  icon: <MailOutlined />,
                },
                {
                  key: USERS_UNLOCK_BY_EMAIL_PATH,
                  label: intl.formatMessage({
                    id: "users.unlock.by-email.title",
                  }),
                  icon: <UnlockOutlined />,
                },
                {
                  key: USERS_FORGOT_PASSWORD_BY_EMAIL_PATH,
                  label: intl.formatMessage({
                    id: "users.forgot-password.by-email.title",
                  }),
                  icon: <IdcardOutlined />,
                },
                {
                  key: LEAVE_WORDS_NEW_PATH,
                  label: intl.formatMessage({
                    id: "leave-words.new.title",
                  }),
                  icon: <MessageOutlined />,
                },
              ]}
            />
            <Footer />
          </Col>
        </Row>
      </Layout.Content>
    </Layout>
  );
};
