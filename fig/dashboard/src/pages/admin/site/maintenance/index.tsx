import { Row, Col } from "antd";

import Smtp from "./Smtp";

const Widget = () => {
  return (
    <Row>
      <Col md={8}>
        <Smtp />
      </Col>
    </Row>
  );
};

export default Widget;
