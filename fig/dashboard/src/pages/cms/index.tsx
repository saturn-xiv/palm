import { Col, Row, Space, Typography, message } from "antd";
import { useCallback, useEffect, useState } from "react";
import { FormattedMessage } from "react-intl";

import { index_page, IPage } from "../../api/cms";
import PageCard from "./pages/Card";
import { IError } from "../../api";
import PaginationBar from "../../components/PaginationBar";
import { DEFAULT_PAGE_SIZE } from "../../components";
import NewPage from "./pages/New";

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const [pages, setPages] = useState<IPage[]>([]);
  const [total, setTotal] = useState(0);

  const reload_pages = useCallback(
    (page: number, size: number) => {
      index_page({ page, size })
        .then((res) => {
          setPages(res.items);
          setTotal(res.pagination.total);
        })
        .catch((reason: IError[]) => {
          messageApi.error(reason.map((x) => x.message).join("\n"));
        });
    },
    [messageApi]
  );

  useEffect(() => {
    reload_pages(1, DEFAULT_PAGE_SIZE);
  }, [messageApi, reload_pages]);

  return (
    <Row gutter={[24, 24]}>
      <Col md={24}>
        <Typography.Title level={3}>
          <FormattedMessage id="pages.cms.index.title" />
        </Typography.Title>
        {contextHolder}
      </Col>
      <Col md={24} style={{ display: "flex", justifyContent: "flex-end" }}>
        <Space align="end">
          <NewPage
            messageApi={messageApi}
            handleReload={() => reload_pages(1, DEFAULT_PAGE_SIZE)}
          />
        </Space>
      </Col>
      {pages.map((x) => (
        <Col key={x.id} md={5}>
          <PageCard
            item={x}
            messageApi={messageApi}
            handleReload={() => reload_pages(1, DEFAULT_PAGE_SIZE)}
          />
        </Col>
      ))}
      <Col md={24}>
        <PaginationBar
          handleChange={(page, size) => {
            reload_pages(page, size);
          }}
          defaultCurrent={1}
          total={total}
        />
      </Col>
    </Row>
  );
};

export default Widget;
