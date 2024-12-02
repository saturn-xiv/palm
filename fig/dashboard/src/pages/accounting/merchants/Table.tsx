import { Space, Table, Typography } from "antd";
import { FormattedMessage } from "react-intl";
import { useCallback, useEffect, useState } from "react";
import type { MessageInstance } from "antd/es/message/interface";

import { IError } from "../../../api";
import {
  ILedger,
  IMerchant,
  index_merchant_by_ledger,
} from "../../../api/hyacinth";
import Memo from "../../../components/Memo";
import Form from "./Form";
import ShowAddress from "../../../components/postal/address/Show";
import ShowRecipient from "../../../components/postal/recipient/Show";

interface IProps {
  ledger: ILedger;
  messageApi: MessageInstance;
}

const Widget = ({ ledger, messageApi }: IProps) => {
  const [items, setItems] = useState<IMerchant[]>([]);

  const handleRefresh = useCallback(
    (id: number) => {
      index_merchant_by_ledger(id)
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
    <Table<IMerchant>
      rowKey="id"
      title={() => (
        <Space>
          <Typography.Title level={4}>
            <FormattedMessage id="pages.accounting.merchants.index.title" />
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
          title: <FormattedMessage id="form.fields.label.label" />,
          dataIndex: "label",
          key: "label",
        },
        {
          title: <FormattedMessage id="form.fields.memo.label" />,
          key: "memo",
          render: (_, { memo }) => <Memo text={memo} />,
        },
        {
          title: <FormattedMessage id="components.postal.address-form.title" />,
          key: "address",
          render: (_, { address }) => address && <ShowAddress item={address} />,
        },
        {
          title: (
            <FormattedMessage id="components.postal.recipient-form.title" />
          ),
          key: "contact",
          render: (_, { contact }) =>
            contact && <ShowRecipient item={contact} />,
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
