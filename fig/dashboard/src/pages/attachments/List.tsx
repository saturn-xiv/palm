import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { FormattedMessage } from "react-intl";
import { Typography } from "antd";

import { IAttachment, index_attachment } from "../../api/daffodil";

interface IParams {
  key: string;
}

const Widget = () => {
  const ref = useRef<ActionType>();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.users.logs.title" />
      </Typography.Title>
      <ProTable<IAttachment, IParams>
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
            title: <FormattedMessage id="form.fields.title.label" />,
            dataIndex: "title",
            key: "title",
          },
          {
            title: <FormattedMessage id="form.fields.content-type.label" />,
            dataIndex: "contentType",
            key: "contentType",
          },
          {
            title: (
              <>
                <FormattedMessage id="form.fields.size.label" />
                (KB)
              </>
            ),
            dataIndex: "size",
            key: "size",
          },
          {
            title: <FormattedMessage id="form.fields.updated-at.label" />,
            dataIndex: "updatedAt",
            key: "updatedAt",
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
            const it = await index_attachment({
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
