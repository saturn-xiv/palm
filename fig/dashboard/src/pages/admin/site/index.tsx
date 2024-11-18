import { Row, Col } from "antd";

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

const Widget = () => {
  const site = useAppSelector(siteInfo);
  return (
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
      <Col md={8}>
        <CnIcp />
      </Col>
      <Col md={8}>
        <CnMps />
      </Col>
      <Col md={8}>
        <Smtp />
      </Col>
      <Col md={8}>
        <Google />
      </Col>
      <Col md={8}>
        <IndexNow />
      </Col>
    </Row>
  );
};

export default Widget;
