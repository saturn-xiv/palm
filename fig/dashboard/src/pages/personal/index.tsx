import { Col, Row } from "antd";

import CancelAccount from "./CancelAccount";
import ChangePassword from "./ChangePassword";
import Logs from "./Logs";
import UpdateProfile from "./UpdateProfile";

const Widget = () => {
  return (
    <Row gutter={24}>
      <Col md={8}>
        <UpdateProfile />
      </Col>
      <Col md={8}>
        <ChangePassword />
      </Col>
      <Col md={8}>
        <CancelAccount />
      </Col>
      <Col md={24}>
        <Logs />
      </Col>
    </Row>
  );
};

export default Widget;
