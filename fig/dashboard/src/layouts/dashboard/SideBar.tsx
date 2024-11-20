import {
  DashboardOutlined,
  UserOutlined,
  InfoCircleOutlined,
  FileOutlined,
} from "@ant-design/icons";
import { useNavigate } from "react-router-dom";

import { Layout, Menu } from "antd";
import { currentUser } from "../../reducers/current-user";
import { useAppSelector } from "../../hooks";

const { Sider } = Layout;

const menu_icon = (key: string) => {
  switch (key) {
    case "/personal":
      return <UserOutlined />;
    case "/admin":
      return <DashboardOutlined />;
    case "/attachments":
      return <FileOutlined />;
    default:
      return <InfoCircleOutlined />;
  }
};

interface IProps {
  collapsed: boolean;
}

const Widget = ({ collapsed }: IProps) => {
  const current_user = useAppSelector(currentUser);
  const navigate = useNavigate();
  return (
    <Sider trigger={null} collapsible collapsed={collapsed}>
      <div className="dashboard-sidebar-logo-vertical" />
      <Menu
        theme="dark"
        mode="inline"
        defaultSelectedKeys={[]}
        onClick={(e) => {
          navigate(`/dashboard/${e.key}`);
        }}
        items={current_user?.sideBar.map((x) => {
          return {
            key: x.to,
            label: x.label,
            icon: menu_icon(x.to),
            children: x.children
              ? x.children.map((y) => {
                  return {
                    key: y.to,
                    label: y.label,
                  };
                })
              : undefined,
          };
        })}
      />
    </Sider>
  );
};

export default Widget;
