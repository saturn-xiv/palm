import { Button, Space, Tooltip, Typography } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { useRef } from "react";
import type { MessageInstance } from "antd/es/message/interface";
import { ActionType, ProTable } from "@ant-design/pro-components";

import {
  IEntry,
  ILedger,
  index_entries_by_ledger,
  update_entry,
} from "../../../api/hyacinth";
import Money from "../../../components/Money";
import { ICurrency } from "../../../api/daffodil";
import EntryForm from "../entries/Form";
import { IError } from "../../../api";
import Bills from "./Bills";

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
    <ProTable<IEntry, IParams>
      title={() => (
        <Space align="baseline">
          <Typography.Title level={4}>
            <FormattedMessage id="pages.accounting.entries.index.title" />
          </Typography.Title>
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
          title: (
            <FormattedMessage id="pages.accounting.entries.form.fields.from-account.label" />
          ),
          key: "from-account",
          render: (_, { fromAccount }) => (
            <Tooltip title={fromAccount.memo}>{fromAccount.label}</Tooltip>
          ),
        },
        {
          title: (
            <FormattedMessage id="pages.accounting.entries.form.fields.to-account.label" />
          ),
          key: "to-account",
          render: (_, { toAccount }) => (
            <Tooltip title={toAccount.memo}>{toAccount.label}</Tooltip>
          ),
        },
        {
          title: <FormattedMessage id="form.fields.category.label" />,
          key: "category",
          render: (_, { category }) => (
            <Button type="text" size="small">
              {category.label}
            </Button>
          ),
        },
        {
          title: <FormattedMessage id="form.fields.amount.label" />,
          key: "category",
          render: (_, { fromAccount, amount }) => (
            <Space>
              <Money currency={fromAccount.currency} amount={amount} />
            </Space>
          ),
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
              <Bills item={item} />
              <EntryForm
                item={item}
                ledger={ledger}
                currencies={currencies}
                title={intl.formatMessage(
                  { id: "pages.accounting.entries.edit.title" },
                  { name: item.id }
                )}
                handleSave={async (values) => {
                  const ok = await update_entry(item.id, values)
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
          const it = await index_entries_by_ledger(ledger.id, {
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
