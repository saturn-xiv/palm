import { Row, Col } from "antd";
import { useEffect, useState } from "react";

import Form from "./Form";
import Users from "./Users";
import { administrators, IUserSelectOption } from "../../../../api/daffodil";

const Widget = () => {
  const [users, setUsers] = useState<IUserSelectOption[]>([]);
  const handleRefresh = () => {
    administrators().then((res) => {
      setUsers(res);
    });
  };
  useEffect(() => {
    handleRefresh();
  }, []);
  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={8}>
        <Form handleRefresh={handleRefresh} />
      </Col>
      <Col md={8}>
        <Users items={users} />
      </Col>
    </Row>
  );
};

export default Widget;
