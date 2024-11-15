import { Row, Col } from "antd";

import ByEmail from "./ByEmail";

const Widget = () => {
  return (
    <Row>
      <Col md={24}>
        <ByEmail />
      </Col>
    </Row>
  );
};

export default Widget;
