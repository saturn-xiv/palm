import { Col, Row, Space, Typography, message } from "antd";
import { useCallback, useEffect, useState } from "react";
import { FormattedMessage } from "react-intl";

import LedgerCard from "./ledgers/Card";
import { IError } from "../../api";
import NewLedger from "./ledgers/New";
import { ILedger, index_ledger } from "../../api/hyacinth";

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const [ledgers, setLedgers] = useState<ILedger[]>([]);

  const reload_ledgers = useCallback(() => {
    index_ledger()
      .then((res) => {
        setLedgers(res);
      })
      .catch((reason: IError[]) => {
        messageApi.error(reason.map((x) => x.message).join("\n"));
      });
  }, [messageApi]);

  useEffect(() => {
    reload_ledgers();
  }, [messageApi, reload_ledgers]);

  return (
    <Row gutter={[24, 24]}>
      <Col md={24}>
        <Typography.Title level={3}>
          <FormattedMessage id="pages.accounting.ledgers.index.title" />
        </Typography.Title>
        {contextHolder}
      </Col>
      <Col md={24} style={{ display: "flex", justifyContent: "flex-end" }}>
        <Space align="end">
          <NewLedger
            messageApi={messageApi}
            handleReload={() => reload_ledgers()}
          />
        </Space>
      </Col>
      {ledgers.map((x) => (
        <Col key={x.id} md={8}>
          <LedgerCard
            item={x}
            messageApi={messageApi}
            handleReload={() => reload_ledgers()}
          />
        </Col>
      ))}
    </Row>
  );
};

export default Widget;
