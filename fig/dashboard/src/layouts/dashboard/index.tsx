import { Outlet } from "react-router-dom";
import {
  GithubFilled,
  InfoCircleFilled,
  QuestionCircleFilled,
} from "@ant-design/icons";
import { PageContainer, ProLayout } from "@ant-design/pro-components";
import { useState } from "react";

import Footer from "../Footer";
import MenuFooter from "./MenuFooter";
import MenuCard from "./MenuCard";
import SearchInput from "./SearchInput";
import app_list from "./app-list";
import route from "./route";
import SignOut from "./SignOut";
import { useAppSelector } from "../../hooks";
import { currentUser as selectCurrentUser } from "../../reducers/current-user";
import { siteInfo as selectSiteInfo } from "../../reducers/site-info";

export const Component = () => {
  const [pathname, setPathname] = useState("/list/sub-page/sub-sub-page1");
  const current_user = useAppSelector(selectCurrentUser);
  const site_info = useAppSelector(selectSiteInfo);

  return (
    <ProLayout
      fixSiderbar
      layout="mix"
      splitMenus
      prefixCls="my-prefix"
      title={site_info.subhead}
      logo={site_info.favicon}
      route={route}
      appList={app_list}
      location={{
        pathname,
      }}
      token={{
        header: {
          colorBgMenuItemSelected: "rgba(0,0,0,0.04)",
        },
      }}
      siderMenuType="group"
      menu={{
        collapsedShowGroupTitle: true,
      }}
      avatarProps={{
        src: current_user.avatar,
        size: "small",
        title: current_user.realName,
        render: (_props, dom) => {
          return <SignOut>{dom}</SignOut>;
        },
      }}
      actionsRender={(props) => {
        if (props.isMobile) return [];
        if (typeof window === "undefined") return [];
        return [
          props.layout !== "side" && document.body.clientWidth > 1400 ? (
            <SearchInput />
          ) : undefined,
          <InfoCircleFilled key="InfoCircleFilled" />,
          <QuestionCircleFilled key="QuestionCircleFilled" />,
          <GithubFilled key="GithubFilled" />,
        ];
      }}
      headerTitleRender={(logo, title, _) => {
        const defaultDom = (
          <a>
            {logo}
            {title}
          </a>
        );
        if (typeof window === "undefined") return defaultDom;
        if (document.body.clientWidth < 1400) {
          return defaultDom;
        }
        if (_.isMobile) return defaultDom;
        return (
          <>
            {defaultDom}
            <MenuCard />
          </>
        );
      }}
      menuFooterRender={(props) => {
        if (props?.collapsed) {
          return undefined;
        }
        return <MenuFooter />;
      }}
      footerRender={() => <Footer />}
      onMenuHeaderClick={(e) => console.log(e)}
      menuItemRender={(item, dom) => (
        <div
          onClick={() => {
            setPathname(item.path || "/welcome");
          }}
        >
          {dom}
        </div>
      )}
    >
      <PageContainer
        token={{
          paddingInlinePageContainerContent: 40,
        }}
        extra={[]}
        footer={[]}
      >
        <Outlet />
      </PageContainer>
    </ProLayout>
  );
};
