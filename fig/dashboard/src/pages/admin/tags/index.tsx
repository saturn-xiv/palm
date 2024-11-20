import { Row, Col } from "antd";

import List from "./List";

const Widget = () => {
  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={8}>
        <List />
      </Col>
    </Row>
  );
};

export default Widget;
