import { Outlet } from "react-router-dom";
import { Layout, theme } from "antd";

import Copyright from "../Copyright";
import SideBar from "./SideBar";

const { Header, Content, Footer } = Layout;

const Widget = () => {
  const {
    token: { colorBgContainer },
  } = theme.useToken();

  return (
    <Layout hasSider>
      <SideBar />
      <Layout style={{ marginInlineStart: 200 }}>
        <Header style={{ padding: 0, background: colorBgContainer }} />
        <Content style={{ margin: "24px 16px 0", overflow: "initial" }}>
          <Outlet />
        </Content>
        <Footer style={{ textAlign: "center" }}>
          <Copyright />
        </Footer>
      </Layout>
    </Layout>
  );
};

export default Widget;
