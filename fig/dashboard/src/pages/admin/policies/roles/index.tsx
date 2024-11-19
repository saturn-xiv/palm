import { Row, Col } from "antd";
import { useEffect, useState } from "react";

import Form from "./Form";
import Relations from "./Relations";
import {
  IUserRoleRelation,
  policy_user_role_relations,
} from "../../../../api/daffodil";

const Widget = () => {
  const [relations, setRelations] = useState<IUserRoleRelation[]>([]);
  const handleRefresh = () => {
    policy_user_role_relations().then((res) => {
      setRelations(res);
    });
  };
  useEffect(() => {
    handleRefresh();
  }, []);
  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={6}>
        <Form handleRefresh={handleRefresh} />
      </Col>
      <Col md={18}>
        <Relations items={relations} />
      </Col>
    </Row>
  );
};

export default Widget;
