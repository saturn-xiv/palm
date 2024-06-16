import { useRef } from "react";
import type { ProColumns } from "@ant-design/pro-components";
import { ProTable, ActionType } from "@ant-design/pro-components";
import { FormattedMessage } from "react-intl";

import { index_attachment, IAttachment } from "../../../api/attachments";
import ShareButton from "./ShareButton";
import EditButton from "./EditButton";
import UploadButton from "./UploadButton";
import DeleteButton from "./DeleteButton";

const Widget = () => {
  const ref = useRef<ActionType>();
  const onRefresh = () => {
    if (ref.current?.reloadAndRest) {
      ref.current?.reloadAndRest();
    }
  };
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
      title: <FormattedMessage id="form.fields.operation.label" />,
      valueType: "option",
      key: "operation",
      render: (_text, record) => {
        const items = [
          <EditButton
            handleRefresh={onRefresh}
            key={`edit-${record.id}`}
            item={record}
          />,
        ];
        if (record.uploadedAt) {
          items.push(<ShareButton key={`share-${record.id}`} item={record} />);
        }
        if (!record.deletedAt) {
          items.push(
            <DeleteButton
              handleRefresh={onRefresh}
              key={`delete-${record.id}`}
              item={record}
            />
          );
        }
        return items;
      },
    },
  ];
  return (
    <ProTable<IAttachment>
      actionRef={ref}
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
      toolBarRender={() => {
        return [<UploadButton handleRefresh={onRefresh} key="upload" />];
      }}
    />
  );
};

export default Widget;
