import { useEffect } from "react";
import { DefaultFooter } from "@ant-design/pro-components";

import { useAppSelector, useAppDispatch } from "../hooks";
import {
  layout2state,
  refresh as refreshLayout,
  siteInfo as selectSiteInfo,
} from "../reducers/site-info";
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
import { HOST as GRPC_HOST, metadata as grpc_metadata } from "../api/grpc";
import { SiteClient } from "../protocols/lilac/AuthServiceClientPb";
import { SiteLayoutRequest } from "../protocols/lilac/auth_pb";

const Widget = () => {
  const site_info = useAppSelector(selectSiteInfo);
  const is_signed_in = useAppSelector(selectIsSignedIn);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (site_info.languages.length === 0) {
      const client = new SiteClient(GRPC_HOST);
      const request = new SiteLayoutRequest();
      request.setLang(get_locale());
      client.layout(request, grpc_metadata()).then((res) => {
        const data = layout2state(res);
        dispatch(refreshLayout(data));
      });
    }
    const token = get_token();
    if (!is_signed_in && token !== null) {
      // current_user()
      //   .then((res) => {
      //     dispatch(refreshUser(res));
      //   })
      //   .catch(() => {
      //     dispatch(signOut());
      //   });
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

  if (site_info.icp) {
    links.push({
      key: "icp-code",
      href: "https://beian.miit.gov.cn/",
      title: <>{site_info.icp}</>,
      blankTarget: true,
    });
  }
  if (site_info.gab) {
    links.push({
      key: "gab-code",
      href: `https://beian.mps.gov.cn/#/query/webSearch?code=${site_info.gab.code}`,
      title: (
        <>
          <img alt="gab" src={icon_img} style={{ width: "12px" }} />
          &nbsp;
          {site_info.gab.name}
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
