import { Row, Col } from "antd";

import Smtp from "./Smtp";

const Widget = () => {
  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={8}>
        <Smtp />
      </Col>
    </Row>
  );
};

export default Widget;
