import type { ProColumns } from "@ant-design/pro-components";
import { ProTable } from "@ant-design/pro-components";
import { FormattedMessage } from "react-intl";

import { index_attachment, IAttachment } from "../../../api/camelia";
import ShareButton from "./ShareButton";
import EditButton from "./EditButton";

const columns: ProColumns<IAttachment>[] = [
  {
    title: <FormattedMessage id="form.fields.id.label" />,
    dataIndex: "id",
    ellipsis: true,
    hideInSearch: true,
  },
  {
    title: <FormattedMessage id="form.fields.content-type.label" />,
    dataIndex: "contentType",
    ellipsis: true,
    sorter: true,
    hideInSearch: true,
  },
  {
    title: <FormattedMessage id="form.fields.title.label" />,
    dataIndex: "title",
    ellipsis: true,
    sorter: true,
    hideInSearch: true,
  },
  {
    title: <FormattedMessage id="form.fields.updated-at.label" />,
    dataIndex: "updatedAt",
    valueType: "dateTime",
    sorter: true,
    hideInSearch: true,
  },
  {
    title: "操作",
    valueType: "option",
    key: "operation",
    render: (_text, record) => [
      <EditButton key={`edit-${record.id}`} item={record} />,
      <ShareButton key={`share-${record.id}`} item={record} />,
    ],
  },
];

const Widget = () => {
  return (
    <ProTable<IAttachment>
      columns={columns}
      cardBordered
      request={async (params) => {
        if (params.current && params.pageSize) {
          const res = await index_attachment(params.current, params.pageSize);
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
