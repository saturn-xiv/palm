import { useRef } from "react";
import type { ProColumns, ActionType } from "@ant-design/pro-components";
import { ProTable } from "@ant-design/pro-components";
import { FormattedMessage } from "react-intl";

import { index_locale, ILocale } from "../../../api/camelia";
import { useAppSelector } from "../../../hooks";
import { currentUser as selectCurrentUser } from "../../../reducers/current-user";
import Form from "./Form";
import DeleteButton from "./DeleteButton";

const columns: ProColumns<ILocale>[] = [
  {
    title: <FormattedMessage id="form.fields.id.label" />,
    dataIndex: "id",
    ellipsis: true,
    hideInSearch: true,
  },
  {
    title: <FormattedMessage id="form.fields.language.label" />,
    valueType: "option",
    key: "lang",
    render: (_text, record) => [
      <FormattedMessage
        key={`col-lang-${record.id}`}
        id={`languages.${record.lang}`}
      />,
    ],
  },
  {
    title: <FormattedMessage id="form.fields.code.label" />,
    dataIndex: "code",
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
    render: (_text, record, _index, action) => [
      <Form
        key={`edit-${record.id}`}
        item={{
          lang: record.lang,
          code: record.code,
          message: record.message,
        }}
        edit
        handleRefresh={() => {
          action?.reload();
        }}
      />,
      <DeleteButton
        key={`delete-${record.id}`}
        item={record}
        handleRefresh={() => {
          action?.reload();
        }}
      />,
    ],
  },
];

const Widget = () => {
  const actionRef = useRef<ActionType>();
  const current_user = useAppSelector(selectCurrentUser);
  return (
    <ProTable<ILocale>
      columns={columns}
      actionRef={actionRef}
      cardBordered
      request={async (params) => {
        if (params.current && params.pageSize) {
          const res = await index_locale(params.current, params.pageSize);
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
        return [
          <Form
            key="locale.new"
            item={{
              lang: current_user.lang,
              code: "",
              message: "",
            }}
            handleRefresh={() => {
              actionRef.current?.reload();
            }}
          />,
        ];
      }}
    />
  );
};

export default Widget;
