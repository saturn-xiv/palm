import { Outlet } from "react-router-dom";
import { Menu, Col, Row, Flex, Layout } from "antd";
import { useNavigate } from "react-router-dom";
import { FormattedMessage } from "react-intl";
import {
  UserAddOutlined,
  MailOutlined,
  UnlockOutlined,
  LoginOutlined,
  CommentOutlined,
  IdcardOutlined,
} from "@ant-design/icons";

import Copyright from "../Copyright";
import { useAppSelector } from "../../hooks";
import { siteInfo } from "../../reducers/site";

const { Header, Footer, Content } = Layout;

const Widget = () => {
  const site = useAppSelector(siteInfo);
  const navigate = useNavigate();

  return (
    <Flex gap="middle" wrap>
      <Layout>
        <Header
          style={{
            color: "#fff",
            height: 64,
            paddingInline: 48,
            lineHeight: "64px",
          }}
        >
          {site?.title} | {site?.subhead}
        </Header>
        <Content>
          <Row>
            <Col offset={6} span={12}>
              <Outlet />
              <br />
              <Menu
                onClick={(it) => {
                  navigate(it.key);
                }}
                defaultSelectedKeys={["/users/sign-in"]}
                mode="inline"
                items={[
                  {
                    key: "/anonymous/users/sign-in",
                    icon: <LoginOutlined />,
                    label: <FormattedMessage id="pages.users.sign-in.title" />,
                  },
                  {
                    key: "/anonymous/users/sign-up",
                    icon: <UserAddOutlined />,
                    label: <FormattedMessage id="pages.users.sign-up.title" />,
                  },
                  {
                    key: "/anonymous/users/forgot-password",
                    icon: <IdcardOutlined />,
                    label: (
                      <FormattedMessage id="pages.users.forgot-password.title" />
                    ),
                  },
                  {
                    key: "/anonymous/users/confirm",
                    icon: <MailOutlined />,
                    label: <FormattedMessage id="pages.users.confirm.title" />,
                  },
                  {
                    key: "/anonymous/users/unlock",
                    icon: <UnlockOutlined />,
                    label: <FormattedMessage id="pages.users.unlock.title" />,
                  },
                  {
                    key: "/anonymous/leave-words/new",
                    icon: <CommentOutlined />,
                    label: (
                      <FormattedMessage id="pages.leave-words.new.title" />
                    ),
                  },
                ]}
              />
            </Col>
          </Row>
        </Content>
        <Footer
          style={{
            textAlign: "center",
          }}
        >
          <Copyright />
        </Footer>
      </Layout>
    </Flex>
  );
};

export default Widget;
