import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { FormattedMessage } from "react-intl";
import { Typography } from "antd";

import { ILog, index_log } from "../../api/daffodil";

interface IParams {
  key: string;
}

const Widget = () => {
  const ref = useRef<ActionType>();
  return (
    <ProTable<ILog, IParams>
      title={() => (
        <Typography.Title level={3}>
          <FormattedMessage id="pages.users.logs.title" />
        </Typography.Title>
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
          title: <FormattedMessage id="form.fields.plugin.label" />,
          dataIndex: "plugin",
          key: "plugin",
        },
        {
          title: <FormattedMessage id="form.fields.level.label" />,
          dataIndex: "level",
          key: "level",
        },
        {
          title: <FormattedMessage id="form.fields.ip.label" />,
          dataIndex: "ip",
          key: "ip",
        },
        {
          title: <FormattedMessage id="form.fields.resource.label" />,
          key: "resource",
          render: (_, { resource }) => (
            <>
              {resource.type}://{resource.id || ""}
            </>
          ),
        },
        {
          title: <FormattedMessage id="form.fields.message.label" />,
          dataIndex: "message",
          key: "message",
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
          const it = await index_log({
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
