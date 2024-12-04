import { Space, Typography } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useRef } from "react";
import type { MessageInstance } from "antd/es/message/interface";
import { ActionType, ProTable } from "@ant-design/pro-components";

import {
  create_entry,
  ILedger,
  index_transaction_by_ledger,
  ITransaction,
} from "../../../api/hyacinth";
import Memo from "../../../components/Memo";
import Form from "./Form";
import { ICurrency } from "../../../api/daffodil";
import EntryForm from "../entries/Form";
import { IError } from "../../../api";

interface IProps {
  ledger: ILedger;
  messageApi: MessageInstance;
  currencies: ICurrency[];
}

interface IParams {
  key: string;
}

const Widget = ({ ledger, currencies, messageApi }: IProps) => {
  const ref = useRef<ActionType>();
  const intl = useIntl();

  return (
    <ProTable<ITransaction, IParams>
      title={() => (
        <Space align="baseline">
          <Typography.Title level={4}>
            <FormattedMessage id="pages.accounting.transactions.index.title" />
          </Typography.Title>
          <Form
            handleRefresh={() => ref.current?.reload()}
            ledger={ledger}
            messageApi={messageApi}
          />
        </Space>
      )}
      bordered
      actionRef={ref}
      params={{ key: "" }}
      search={false}
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
              <EntryForm
                ledger={ledger}
                currencies={currencies}
                title={intl.formatMessage(
                  {
                    id: "pages.accounting.entries.new.title",
                  },
                  { name: item.memo }
                )}
                handleSave={async (values) => {
                  const ok = await create_entry(item.id, values)
                    .then(async () => {
                      await messageApi.success(
                        intl.formatMessage({ id: "flashes.succeed" })
                      );
                      ref.current?.reload();
                      return true;
                    })
                    .catch((reason: IError[]) => {
                      messageApi.error(reason.map((x) => x.message).join("\n"));
                      return false;
                    });
                  return ok;
                }}
              />
              <Form
                handleRefresh={() => ref.current?.reload()}
                ledger={ledger}
                messageApi={messageApi}
                item={item}
              />
            </Space>
          ),
        },
      ]}
      rowKey="id"
      request={async (
        params: { key: string } & {
          pageSize?: number;
          current?: number;
        }
      ) => {
        if (params.pageSize && params.current) {
          const it = await index_transaction_by_ledger(ledger.id, {
            page: params.current,
            size: params.pageSize,
          });
          return {
            data: it.items,
            success: true,
            total: it.pagination.total,
          };
        }
        return { data: [], success: false, total: 0 };
      }}
    />
  );
};

export default Widget;
