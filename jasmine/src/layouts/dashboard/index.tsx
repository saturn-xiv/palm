import { Outlet } from "react-router-dom";
import {
  GithubFilled,
  InfoCircleFilled,
  QuestionCircleFilled,
} from "@ant-design/icons";
import {
  MenuDataItem,
  PageContainer,
  ProLayout,
} from "@ant-design/pro-components";
import { useNavigate } from "react-router-dom";

import Footer from "../Footer";
import MenuFooter from "./MenuFooter";
import MenuCard from "./MenuCard";
import SearchInput from "./SearchInput";
import app_list from "./app-list";
import SignOut from "./SignOut";
import { useAppSelector, useAppDispatch } from "../../hooks";
import { currentUser as selectCurrentUser } from "../../reducers/current-user";
import { siteInfo as selectSiteInfo } from "../../reducers/site-info";
import {
  set_pathname,
  sideBar as selectSideBar,
} from "../../reducers/side-bar";
import { routes as fetch_routes, IRoute } from "../../api/camelia";

const to_route_menu = (rt: IRoute): MenuDataItem => {
  const it: MenuDataItem = {
    name: rt.name,
    path: rt.path,
    children: rt.routes.map(to_route_menu),
  };
  return it;
};

export const Component = () => {
  const current_user = useAppSelector(selectCurrentUser);
  const site_info = useAppSelector(selectSiteInfo);
  const side_bar = useAppSelector(selectSideBar);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();

  return (
    <ProLayout
      fixSiderbar
      layout="mix"
      splitMenus
      prefixCls="my-prefix"
      title={site_info.subhead}
      logo={site_info.favicon}
      appList={app_list}
      location={{
        pathname: side_bar.pathname,
      }}
      token={{
        header: {
          colorBgMenuItemSelected: "rgba(0,0,0,0.04)",
        },
      }}
      siderMenuType="group"
      menu={{
        collapsedShowGroupTitle: true,
        request: async () => {
          const items = await fetch_routes();
          // console.log(items);
          return items.map(to_route_menu);
        },
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
          <a href="/">
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
      onMenuHeaderClick={(e) => {
        e.preventDefault();
        window.open("/", "_blank")?.focus();
      }}
      menuItemRender={(item, dom) => (
        <div
          onClick={() => {
            if (item.path) {
              // console.log(item);
              dispatch(set_pathname(item.path));
              navigate(item.path);
            }
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
