import { Row, Col } from "antd";

import Google from "./Google";
import IndexNow from "./IndexNow";

const Widget = () => {
  return (
    <Row>
      <Col md={12}>
        <Google />
      </Col>
      <Col md={12}>
        <IndexNow />
      </Col>
    </Row>
  );
};

export default Widget;
