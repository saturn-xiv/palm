import { Row, Col } from "antd";
import { useEffect } from "react";

const Widget = () => {
  const handleRefresh = () => {};
  useEffect(() => {
    handleRefresh();
  }, []);
  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={6}></Col>
      <Col md={18}></Col>
    </Row>
  );
};

export default Widget;
