import { useEffect } from "react";
import { DefaultFooter } from "@ant-design/pro-components";

import { useAppSelector, useAppDispatch } from "../hooks";
import {
  refresh as refreshLayout,
  siteInfo as selectSiteInfo,
} from "../reducers/site-info";
import { fetch_layout, current_user } from "../api/camelia";
import { get as get_locale } from "../locales";
import {
  get as get_token,
  refresh as refreshUser,
  isSignedIn as selectIsSignedIn,
  signOut,
} from "../reducers/current-user";
import { home_url } from "../utils";
import icon_img from "../assets/beian/1.jpg";
import SwitchLanguage from "./SwitchLanguage";

const Widget = () => {
  const site_info = useAppSelector(selectSiteInfo);
  const is_signed_in = useAppSelector(selectIsSignedIn);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (site_info.version === "") {
      fetch_layout(get_locale()).then((res) => {
        dispatch(refreshLayout(res));
      });
    }
    const token = get_token();
    if (!is_signed_in && token !== null) {
      current_user()
        .then((res) => {
          dispatch(refreshUser(res));
        })
        .catch(() => {
          dispatch(signOut());
        });
    }
  }, [is_signed_in, dispatch, site_info]);

  const links = [
    {
      key: "home",
      title: <>{site_info.title}</>,
      href: home_url(),
      blankTarget: true,
    },
  ];

  if (site_info.icpCode) {
    links.push({
      key: "icp-code",
      href: "https://beian.miit.gov.cn/",
      title: <>{site_info.icpCode}</>,
      blankTarget: true,
    });
  }
  if (site_info.gabCode) {
    links.push({
      key: "gab-code",
      href: `https://beian.mps.gov.cn/#/query/webSearch?code=${site_info.gabCode.code}`,
      title: (
        <>
          <img alt="gab" src={icon_img} style={{ width: "12px" }} />
          &nbsp;
          {site_info.gabCode.name}
        </>
      ),
      blankTarget: true,
    });
  }
  links.push({
    key: "switch-language",
    href: "/",
    title: (
      <span
        onClick={(e) => {
          e.preventDefault();
        }}
      >
        <SwitchLanguage />
      </span>
    ),
    blankTarget: false,
  });

  return <DefaultFooter links={links} copyright={site_info.copyright} />;
};

export default Widget;
