import { Layout, Button, Row, Col, Card } from "antd";
import { useNavigate } from "react-router-dom";
import { FormattedMessage } from "react-intl";

import Footer from "../layouts/Footer";
import { useAppSelector } from "../hooks";
import { siteInfo as selectSiteInfo } from "../reducers/site-info";
import not_found_img from "../assets/404.svg";

export const Component = () => {
  const navigate = useNavigate();
  const site_info = useAppSelector(selectSiteInfo);
  return (
    <Layout>
      <Layout.Content>
        <Row justify="space-evenly">
          <Col sm={24} md={{ span: 8 }}>
            <Card
              title={site_info.subhead}
              hoverable
              cover={<img alt="not found" src={not_found_img} />}
              actions={[
                <Button
                  type="primary"
                  onClick={() => {
                    navigate("/");
                  }}
                >
                  <FormattedMessage id="buttons.go-home" />
                </Button>,
                <Button
                  onClick={() => {
                    navigate(-1);
                  }}
                >
                  <FormattedMessage id="buttons.go-back" />
                </Button>,
              ]}
            >
              <Card.Meta
                title={<FormattedMessage id="layouts.not-found.title" />}
                description={<FormattedMessage id="layouts.not-found.body" />}
              />
            </Card>
          </Col>
        </Row>
      </Layout.Content>
      <Layout.Footer>
        <Footer />
      </Layout.Footer>
    </Layout>
  );
};
