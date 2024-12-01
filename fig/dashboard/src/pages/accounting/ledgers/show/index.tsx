import { Col, message, Row, Space, Tabs, Typography } from "antd";
import { FormattedMessage } from "react-intl";
import { useNavigate, useParams } from "react-router-dom";
import { HomeOutlined, AccountBookOutlined } from "@ant-design/icons";
import { Breadcrumb } from "antd";
import { useEffect, useState } from "react";

import { ILedger, show_ledger } from "../../../../api/hyacinth";
import { IError } from "../../../../api";
import Logs from "./Logs";
import Accounts from "./accounts/Table";
import Transactions from "./transactions/Table";
import Merchants from "./merchants/Table";
import Categories from "./categories/Table";

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [item, setItem] = useState<ILedger>();

  useEffect(() => {
    if (id) {
      show_ledger(parseInt(id))
        .then((res) => {
          setItem(res);
        })
        .catch((reason: IError[]) => {
          messageApi.error(reason.map((x) => x.message).join("\n"));
          return false;
        });
    }
  }, [id, messageApi]);
  return item ? (
    <Row gutter={[24, 24]}>
      <Col md={24}>
        <Breadcrumb
          items={[
            {
              title: <HomeOutlined />,
              onClick: (e) => {
                e.preventDefault();
                navigate("/dashboard");
              },
            },
            {
              title: (
                <Space>
                  <AccountBookOutlined />
                  <FormattedMessage id="pages.accounting.index.title" />
                </Space>
              ),
              onClick: (e) => {
                e.preventDefault();
                navigate("/dashboard/accounting");
              },
            },
            {
              title: item.label,
            },
          ]}
        />
      </Col>
      <Col md={24}>
        <Typography.Title level={3}>{item.label}</Typography.Title>
        <Typography.Paragraph>{item.memo}</Typography.Paragraph>
        {contextHolder}
      </Col>
      <Col md={24}>
        <Tabs
          defaultActiveKey="logs"
          items={[
            {
              key: "logs",
              label: (
                <FormattedMessage id="pages.accounting.ledgers.tabs.logs.title" />
              ),
              children: <Logs ledger={item} />,
            },
            {
              key: "accounts",
              label: (
                <FormattedMessage id="pages.accounting.ledgers.tabs.accounts.title" />
              ),
              children: <Accounts />,
            },
            {
              key: "merchants",
              label: (
                <FormattedMessage id="pages.accounting.ledgers.tabs.merchants.title" />
              ),
              children: <Merchants />,
            },
            {
              key: "categories",
              label: (
                <FormattedMessage id="pages.accounting.ledgers.tabs.categories.title" />
              ),
              children: <Categories />,
            },
            {
              key: "transactions",
              label: (
                <FormattedMessage id="pages.accounting.ledgers.tabs.transactions.title" />
              ),
              children: <Transactions />,
            },
          ]}
        />
      </Col>
    </Row>
  ) : (
    <></>
  );
};

export default Widget;
