import { Row, Col } from "antd";
import { useEffect, useState } from "react";

import List from "./List";
import Tree from "./Tree";
import { ICategory, index_category } from "../../../api/daffodil";

const Widget = () => {
  const [items, setItems] = useState<ICategory[]>([]);
  const [root, setRoot] = useState<ICategory>();
  const handleRefresh = () => {
    index_category().then((res) => setItems(res));
  };
  useEffect(() => {
    handleRefresh();
  }, []);
  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={8}>
        <List
          handleShow={setRoot}
          nodes={items}
          handleRefresh={handleRefresh}
        />
      </Col>
      <Col md={16}>
        <Tree item={root} nodes={items} />
      </Col>
    </Row>
  );
};

export default Widget;
