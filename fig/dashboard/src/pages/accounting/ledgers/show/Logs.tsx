import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { FormattedMessage } from "react-intl";
import { Typography } from "antd";

import { ILedger, ILog, index_log_by_ledger } from "../../../../api/hyacinth";

interface IProps {
  ledger: ILedger;
}

interface IParams {
  key: string;
}

const Widget = ({ ledger }: IProps) => {
  const ref = useRef<ActionType>();
  return (
    <>
      <Typography.Title level={4}>
        <FormattedMessage id="pages.users.logs.title" />
      </Typography.Title>
      <ProTable<ILog, IParams>
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
            title: <FormattedMessage id="form.fields.user.label" />,
            dataIndex: "username",
            key: "username",
          },
          {
            title: <FormattedMessage id="form.fields.action.label" />,
            dataIndex: "action",
            key: "action",
          },
          {
            title: <FormattedMessage id="form.fields.memo.label" />,
            dataIndex: "memo",
            key: "memo",
          },
          {
            title: <FormattedMessage id="form.fields.reason.label" />,
            dataIndex: "reason",
            key: "reason",
          },
          {
            title: <FormattedMessage id="form.fields.ip.label" />,
            dataIndex: "ip",
            key: "ip",
          },
          {
            title: <FormattedMessage id="form.fields.created-at.label" />,
            dataIndex: "createdAt",
            key: "createdAt",
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
            const it = await index_log_by_ledger(ledger.id, {
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
    </>
  );
};

export default Widget;
