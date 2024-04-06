import { useState } from "react";
import { Outlet } from "react-router-dom";
import { PageContainer, ProCard, ProLayout } from "@ant-design/pro-components";
import {
  LoginOutlined,
  IdcardOutlined,
  MailOutlined,
  MessageOutlined,
  UserAddOutlined,
  UnlockOutlined,
} from "@ant-design/icons";
import { useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { SIGN_IN_PATH as USER_SIGN_IN_PATH } from "../../reducers/current-user";
import Footer from "../Footer";
import SwitchLanguage from "../SwitchLanguage";
import { useAppSelector } from "../../hooks";
import { siteInfo as selectSiteInfo } from "../../reducers/site-info";

export const Component = () => {
  const navigate = useNavigate();
  const intl = useIntl();
  const site_info = useAppSelector(selectSiteInfo);

  const [pathname, setPathname] = useState(USER_SIGN_IN_PATH);

  return (
    <ProLayout
      title={site_info.subhead}
      logo={site_info.favicon}
      location={{
        pathname,
      }}
      siderMenuType="group"
      menu={{
        collapsedShowGroupTitle: true,
      }}
      actionsRender={(props) => {
        if (props.isMobile) {
          return [];
        }
        if (typeof window === "undefined") return [];
        return [<SwitchLanguage key="language-bar" />];
      }}
      menuItemRender={(item, dom) => (
        <a
          onClick={() => {
            if (item.path) {
              setPathname(item.path);
              navigate(item.path);
            }
          }}
        >
          {dom}
        </a>
      )}
      route={{
        path: "/anonymous",
        routes: [
          {
            path: USER_SIGN_IN_PATH,
            name: intl.formatMessage({ id: "users.sign-in.title" }),
            icon: <LoginOutlined />,
          },
          {
            path: "/anonymous/users/sign-up",
            name: intl.formatMessage({ id: "users.sign-up.title" }),
            icon: <UserAddOutlined />,
          },
          {
            path: "/anonymous/users/confirm",
            name: intl.formatMessage({ id: "users.confirm.by-email.title" }),
            icon: <MailOutlined />,
          },
          {
            path: "/anonymous/users/unlock",
            name: intl.formatMessage({ id: "users.unlock.by-email.title" }),
            icon: <UnlockOutlined />,
          },
          {
            path: "/anonymous/users/forgot-password",
            name: intl.formatMessage({
              id: "users.forgot-password.title",
            }),
            icon: <IdcardOutlined />,
          },
          {
            path: "/anonymous/leave-words/new.title",
            name: intl.formatMessage({
              id: "leave-words.new.title",
            }),
            icon: <MessageOutlined />,
          },
        ],
      }}
      footerRender={() => <Footer />}
    >
      <PageContainer onBack={() => window.history.back()}>
        <ProCard
          style={{
            height: "200vh",
            minHeight: 800,
          }}
        >
          <Outlet />
        </ProCard>
      </PageContainer>
    </ProLayout>
  );
};
