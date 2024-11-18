import { Row, Col } from "antd";

import Google from "./Google";
import IndexNow from "./IndexNow";
import Rss from "./Rss";
import Sitemap from "./Sitemap";

const Widget = () => {
  return (
    <Row gutter={[{ md: 24 }, { md: 24 }]}>
      <Col md={8}>
        <Rss />
      </Col>
      <Col md={8}>
        <Sitemap />
      </Col>
      <Col md={12}>
        <Google />
      </Col>
      <Col md={12}>
        <IndexNow />
      </Col>
    </Row>
  );
};

export default Widget;
