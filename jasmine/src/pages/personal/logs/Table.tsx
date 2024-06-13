import type { ProColumns } from "@ant-design/pro-components";
import { ProTable } from "@ant-design/pro-components";

import { FormattedMessage } from "react-intl";

import { logs as fetch_logs, ILog } from "../../../api/users";

const columns: ProColumns<ILog>[] = [
  {
    title: <FormattedMessage id="form.fields.id.label" />,
    dataIndex: "id",
    ellipsis: true,
    hideInSearch: true,
  },
  {
    title: <FormattedMessage id="form.fields.ip.label" />,
    dataIndex: "ip",
    ellipsis: true,
    sorter: true,
    hideInSearch: true,
  },
  {
    title: <FormattedMessage id="form.fields.message.label" />,
    dataIndex: "message",
    ellipsis: true,
    sorter: true,
    hideInSearch: true,
  },
  {
    title: <FormattedMessage id="form.fields.created-at.label" />,
    dataIndex: "createdAt",
    valueType: "dateTime",
    sorter: true,
    hideInSearch: true,
  },
];

const Widget = () => {
  return (
    <ProTable<ILog>
      columns={columns}
      cardBordered
      request={async (params) => {
        // console.log(params);
        if (params.current && params.pageSize) {
          const res = await fetch_logs(params.current, params.pageSize);
          return {
            data: res.items,
            success: true,
            total: res.pagination.total,
          };
        }
        return {
          data: [],
          success: false,
          total: 0,
        };
      }}
      rowKey="id"
      search={false}
      options={{
        setting: {
          listsHeight: 400,
        },
      }}
      dateFormatter="string"
    />
  );
};

export default Widget;
