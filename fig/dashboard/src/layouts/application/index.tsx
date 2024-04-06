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

import Footer from "../Footer";
import SwitchLanguage from "../SwitchLanguage";
import { useAppSelector, useAppDispatch } from "../../hooks";
import { siteInfo as selectSiteInfo } from "../../reducers/site-info";
import {
  set_pathname,
  sideBar as selectSideBar,
} from "../../reducers/side-bar";
import {
  LEAVE_WORDS_NEW_PATH,
  USERS_CONFIRM_BY_EMAIL_PATH,
  USERS_CONFIRM_BY_TOKEN_PATH,
  USERS_FORGOT_PASSWORD_PATH,
  USERS_RESET_PASSWORD_PATH,
  USERS_SIGN_IN_PATH,
  USERS_SIGN_UP_PATH,
  USERS_UNLOCK_BY_EMAIL_PATH,
  USERS_UNLOCK_BY_TOKEN_PATH,
} from "../../Router";

export const Component = () => {
  const navigate = useNavigate();
  const intl = useIntl();
  const site_info = useAppSelector(selectSiteInfo);
  const side_bar = useAppSelector(selectSideBar);
  const dispatch = useAppDispatch();

  return (
    <ProLayout
      title={site_info.subhead}
      logo={site_info.favicon}
      location={{
        pathname: side_bar.pathname,
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
              dispatch(set_pathname(item.path));
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
            path: USERS_SIGN_IN_PATH,
            name: intl.formatMessage({ id: "users.sign-in.title" }),
            icon: <LoginOutlined />,
          },
          {
            path: USERS_SIGN_UP_PATH,
            name: intl.formatMessage({ id: "users.sign-up.title" }),
            icon: <UserAddOutlined />,
          },
          {
            path: USERS_CONFIRM_BY_EMAIL_PATH,
            name: intl.formatMessage({ id: "users.confirm.by-email.title" }),
            icon: <MailOutlined />,
          },
          {
            path: USERS_CONFIRM_BY_TOKEN_PATH,
            name: intl.formatMessage({ id: "users.confirm.by-token.title" }),
            hideInMenu: true,
          },
          {
            path: USERS_UNLOCK_BY_EMAIL_PATH,
            name: intl.formatMessage({ id: "users.unlock.by-email.title" }),
            icon: <UnlockOutlined />,
          },
          {
            path: USERS_UNLOCK_BY_TOKEN_PATH,
            name: intl.formatMessage({ id: "users.unlock.by-token.title" }),
            hideInMenu: true,
          },
          {
            path: USERS_FORGOT_PASSWORD_PATH,
            name: intl.formatMessage({
              id: "users.forgot-password.title",
            }),
            icon: <IdcardOutlined />,
          },
          {
            path: USERS_RESET_PASSWORD_PATH,
            name: intl.formatMessage({
              id: "users.reset-password.title",
            }),
            hideInMenu: true,
          },
          {
            path: LEAVE_WORDS_NEW_PATH,
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
