import { useEffect } from "react";
import { Col } from "antd";

import { useAppDispatch, useAppSelector } from "../../../../hooks";
import { set_pathname } from "../../../../reducers/side-bar";
import { siteInfo as selectSiteInfo } from "../../../../reducers/site-info";
import { SETTINGS_SITE_SEO_PATH } from "../../../../Router";
import Baidu from "./Baidu";
import Google from "./google/SiteVerification";
import IndexNow from "./IndexNow";
import ReCaptcha from "./google/ReCaptcha";
import Rss from "./Rss";
import Sitemap from "./Sitemap";

export const Component = () => {
  const dispatch = useAppDispatch();
  const site_info = useAppSelector(selectSiteInfo);
  useEffect(() => {
    dispatch(set_pathname(SETTINGS_SITE_SEO_PATH));
  }, [dispatch]);
  return (
    <>
      <Col sm={24} md={8}>
        <Baidu />
      </Col>
      <Col sm={24} md={8}>
        <Google />
      </Col>
      <Col sm={24} md={8}>
        <IndexNow />
      </Col>
      <Col sm={24} md={8}>
        <ReCaptcha />
      </Col>
      <Col sm={24} md={8}>
        <Rss languages={site_info.languages} />
      </Col>
      <Col sm={24} md={8}>
        <Sitemap languages={site_info.languages} />
      </Col>
    </>
  );
};
