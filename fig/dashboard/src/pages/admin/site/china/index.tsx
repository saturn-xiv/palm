import { Row, Col } from "antd";

import CnIcp from "./icp";
import CnMps from "./mps";

const Widget = () => {
  return (
    <Row>
      <Col md={12}>
        <CnIcp />
      </Col>
      <Col md={12}>
        <CnMps />
      </Col>
    </Row>
  );
};

export default Widget;
