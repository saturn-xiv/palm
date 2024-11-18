import { Row, Col } from "antd";

import PostgreSql from "./PostgreSql";
import Redis from "./Redis";
import OpenSearch from "./OpenSearch";
import RabbitMQ from "./RabbitMQ";

const Widget = () => {
  return (
    <Row>
      <Col md={8}>
        <PostgreSql />
      </Col>
      <Col md={8}>
        <Redis />
      </Col>
      <Col md={8}>
        <RabbitMQ />
      </Col>
      <Col md={8}>
        <OpenSearch />
      </Col>
    </Row>
  );
};

export default Widget;
