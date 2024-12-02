import { Space, Table, Typography } from "antd";
import { FormattedMessage } from "react-intl";
import { useCallback, useEffect, useState } from "react";
import type { MessageInstance } from "antd/es/message/interface";

import { IError } from "../../../api";
import {
  IAccount,
  ILedger,
  index_account_by_ledger,
} from "../../../api/hyacinth";
import CreateMain from "./CreateMain";
import CreateSub from "./CreateSub";
import Update from "./Update";
import { ICurrency } from "../../../api/daffodil";

interface IProps {
  ledger: ILedger;
  messageApi: MessageInstance;
  currencies: ICurrency[];
}

const Widget = ({ ledger, currencies, messageApi }: IProps) => {
  const [items, setItems] = useState<IAccount[]>([]);

  const handleRefresh = useCallback(
    (id: number) => {
      index_account_by_ledger(id)
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
    <Table<IAccount>
      rowKey="id"
      title={() => (
        <Space>
          <Typography.Title level={4}>
            <FormattedMessage id="pages.accounting.accounts.index.title" />
          </Typography.Title>
          <CreateMain
            handleRefresh={() => handleRefresh(ledger.id)}
            ledger={ledger}
            currencies={currencies}
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
          title: <FormattedMessage id="form.fields.parent.label" />,
          dataIndex: "parent",
          key: "parent",
        },
        {
          title: <FormattedMessage id="form.fields.label.label" />,
          dataIndex: "label",
          key: "label",
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
              <CreateSub
                parent={item}
                currencies={currencies}
                messageApi={messageApi}
                handleRefresh={() => handleRefresh(ledger.id)}
              />
              <Update
                handleRefresh={() => handleRefresh(ledger.id)}
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
