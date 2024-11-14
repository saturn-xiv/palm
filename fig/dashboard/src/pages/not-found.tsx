import { Space, Col, Button, Row, Flex, Layout, Image } from "antd";
import { useNavigate } from "react-router-dom";
import { FormattedMessage } from "react-intl";
import { HomeOutlined, RollbackOutlined } from "@ant-design/icons";

import page_not_found from "../assets/page-not-found.svg";
import Copyright from "../layouts/Copyright";
import { useAppSelector } from "../hooks";
import { siteInfo } from "../reducers/site";

const { Header, Footer, Content } = Layout;

const Widget = () => {
  const site = useAppSelector(siteInfo);
  const navigate = useNavigate();
  return (
    <Flex gap="middle" wrap>
      <Layout>
        <Header
          style={{
            color: "#fff",
            height: 64,
            paddingInline: 48,
            lineHeight: "64px",
          }}
        >
          {site?.title} | {site?.subhead}
        </Header>
        <Content>
          <Row>
            <Col offset={8} span={8}>
              <Space>
                <Image src={page_not_found} />
              </Space>
              <Space size="large">
                <Button
                  icon={<RollbackOutlined />}
                  onClick={(e) => {
                    e.preventDefault();
                    navigate(-1);
                  }}
                >
                  <FormattedMessage id="buttons.go-back" />
                </Button>
                <Button
                  type="primary"
                  icon={<HomeOutlined />}
                  onClick={(e) => {
                    e.preventDefault();
                    navigate("/");
                  }}
                >
                  <FormattedMessage id="buttons.go-home" />
                </Button>
              </Space>
            </Col>
          </Row>
        </Content>
        <Footer
          style={{
            textAlign: "center",
          }}
        >
          <Copyright />
        </Footer>
      </Layout>
    </Flex>
  );
};

export default Widget;
