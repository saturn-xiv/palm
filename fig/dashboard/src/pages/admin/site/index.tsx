import { Row, Col, Tabs } from "antd";

import CnIcp from "./china/icp";
import CnMps from "./china/mps";
import Smtp from "./Smtp";
import Google from "./Google";
import IndexNow from "./IndexNow";
import BaseInfo from "./BaseInfo";
import Keywords from "./Keywords";
import Author from "./Author";
import { useAppSelector } from "../../../hooks";
import { siteInfo } from "../../../reducers/site";
import { DEFAULT_LANGUAGE } from "../../../i18n";
import { FormattedMessage } from "react-intl";

const Widget = () => {
  const site = useAppSelector(siteInfo);
  return (
    <Tabs
      defaultActiveKey="info"
      items={[
        {
          key: "info",
          label: <FormattedMessage id="pages.admin.site.tabs.info.label" />,
          children: (
            <Row>
              <Col md={8}>
                <BaseInfo lang={site?.locale || DEFAULT_LANGUAGE} />
              </Col>
              <Col md={8}>
                <Author />
              </Col>
              <Col md={8}>
                <Keywords />
              </Col>
            </Row>
          ),
        },
        {
          key: "china",
          label: <FormattedMessage id="pages.admin.site.tabs.china.label" />,
          children: (
            <Row>
              <Col md={12}>
                <CnIcp />
              </Col>
              <Col md={12}>
                <CnMps />
              </Col>
            </Row>
          ),
        },
        {
          key: "seo",
          label: <FormattedMessage id="pages.admin.site.tabs.seo.label" />,
          children: (
            <Row>
              <Col md={12}>
                <Google />
              </Col>
              <Col md={12}>
                <IndexNow />
              </Col>
            </Row>
          ),
        },
        {
          key: "maintenance",
          label: (
            <FormattedMessage id="pages.admin.site.tabs.maintenance.label" />
          ),
          children: (
            <Row>
              <Col md={8}>
                <Smtp />
              </Col>
            </Row>
          ),
        },
      ]}
    />
  );
};

export default Widget;
