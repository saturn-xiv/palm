import { Outlet } from "react-router-dom";
import { Button, Space, Layout, theme } from "antd";
import { useState } from "react";
import { MenuFoldOutlined, MenuUnfoldOutlined } from "@ant-design/icons";

import Copyright from "../Copyright";
import SideBar from "./SideBar";
import { header_button_style } from "./style";
import SignOutButton from "./SignOutButton";

const { Header, Content, Footer } = Layout;

const Widget = () => {
  const [collapsed, setCollapsed] = useState(false);
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  return (
    <Layout hasSider>
      <SideBar collapsed={collapsed} />
      <Layout>
        <Header style={{ padding: 0, background: colorBgContainer }}>
          <Button
            type="text"
            icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
            onClick={() => setCollapsed(!collapsed)}
            style={header_button_style}
          />
          <Space align="center" wrap={false}>
            <SignOutButton />
          </Space>
        </Header>
        <Content
          style={{
            margin: "24px 16px",
            padding: 24,
            minHeight: 280,
            background: colorBgContainer,
            borderRadius: borderRadiusLG,
          }}
        >
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
