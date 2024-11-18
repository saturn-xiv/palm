import { Row, Col } from "antd";

import Table from "./Table";

const Widget = () => {
  return (
    <Row>
      <Col md={24}>
        <Table />
      </Col>
    </Row>
  );
};

export default Widget;
