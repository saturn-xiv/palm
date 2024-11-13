import { LoadingOutlined } from "@ant-design/icons";
import { Space, Col, Row } from "antd";

const Widget = () => {
  return (
    <Row>
      <Col offset={8} span={8}>
        <Space>
          <LoadingOutlined />
        </Space>
      </Col>
    </Row>
  );
};

export default Widget;
