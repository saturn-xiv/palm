import { Space, Table, Typography } from "antd";
import { FormattedMessage } from "react-intl";
import { useCallback, useEffect, useState } from "react";
import type { MessageInstance } from "antd/es/message/interface";

import { IError } from "../../../api";
import {
  ILedger,
  index_transaction_by_ledger,
  ITransaction,
} from "../../../api/hyacinth";
import Memo from "../../../components/Memo";
import Form from "./Form";
import { ICurrency } from "../../../api/daffodil";

interface IProps {
  ledger: ILedger;
  messageApi: MessageInstance;
  currencies: ICurrency[];
}

const Widget = ({ ledger, messageApi }: IProps) => {
  const [items, setItems] = useState<ITransaction[]>([]);

  const handleRefresh = useCallback(
    (id: number) => {
      index_transaction_by_ledger(id)
        .then((res) => {
          setItems(res);
        })
        .catch((reason: IError[]) => {
          messageApi.error(reason.map((x) => x.message).join("\n"));
        });
    },
    [messageApi]
  );
  useEffect(() => {
    handleRefresh(ledger.id);
  }, [handleRefresh, ledger]);
  return (
    <Table<ITransaction>
      rowKey="id"
      title={() => (
        <Space align="baseline">
          <Typography.Title level={4}>
            <FormattedMessage id="pages.accounting.transactions.index.title" />
          </Typography.Title>
          <Form
            handleRefresh={() => handleRefresh(ledger.id)}
            ledger={ledger}
            messageApi={messageApi}
          />
        </Space>
      )}
      columns={[
        {
          title: <FormattedMessage id="form.fields.id.label" />,
          dataIndex: "id",
          key: "id",
        },
        {
          title: <FormattedMessage id="form.fields.memo.label" />,
          key: "memo",
          render: (_, { memo }) => <Memo text={memo} />,
        },
        {
          title: <FormattedMessage id="form.fields.traded-at.label" />,
          key: "tradedAt",
          render: (_, { tradedAt }) => (
            <Space>
              {tradedAt.datetime}
              {tradedAt.timezone}
            </Space>
          ),
        },
        {
          title: <FormattedMessage id="form.fields.updated-at.label" />,
          dataIndex: "updatedAt",
          key: "updatedAt",
        },
        {
          title: <FormattedMessage id="buttons.manage" />,
          key: "manage",
          render: (_, item) => (
            <Space>
              <Form
                handleRefresh={() => handleRefresh(ledger.id)}
                ledger={ledger}
                messageApi={messageApi}
                item={item}
              />
            </Space>
          ),
        },
      ]}
      dataSource={items}
    />
  );
};

export default Widget;
