import { Row, Col } from "antd";

import Base from "./Base";
import Keywords from "./Keywords";
import Author from "./Author";
import { useAppSelector } from "../../../../hooks";
import { siteInfo } from "../../../../reducers/site";
import { DEFAULT_LANGUAGE } from "../../../../i18n";

const Widget = () => {
  const site = useAppSelector(siteInfo);
  return (
    <Row>
      <Col md={8}>
        <Base lang={site?.locale || DEFAULT_LANGUAGE} />
      </Col>
      <Col md={8}>
        <Author />
      </Col>
      <Col md={8}>
        <Keywords />
      </Col>
    </Row>
  );
};

export default Widget;
