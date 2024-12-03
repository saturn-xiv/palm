import { Space, Table, Typography } from "antd";
import { FormattedMessage } from "react-intl";
import { useCallback, useEffect, useState } from "react";
import type { MessageInstance } from "antd/es/message/interface";

import { IError } from "../../../api";
import {
  ICategory,
  ILedger,
  index_category_by_ledger,
} from "../../../api/hyacinth";
import Create from "./Create";
import Update from "./Update";

interface IProps {
  ledger: ILedger;
  messageApi: MessageInstance;
}

const Widget = ({ ledger, messageApi }: IProps) => {
  const [items, setItems] = useState<ICategory[]>([]);

  const handleRefresh = useCallback(
    (id: number) => {
      index_category_by_ledger(id)
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
    <Table<ICategory>
      rowKey="id"
      title={() => (
        <Space align="baseline">
          <Typography.Title level={4}>
            <FormattedMessage id="pages.accounting.categories.index.title" />
          </Typography.Title>
          <Create
            handleRefresh={() => handleRefresh(ledger.id)}
            ledger={ledger}
            items={items}
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
          title: <FormattedMessage id="form.fields.parent.label" />,
          dataIndex: "parent",
          key: "parent",
        },
        {
          title: <FormattedMessage id="form.fields.label.label" />,
          dataIndex: "label",
          key: "label",
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
              <Update
                handleRefresh={() => handleRefresh(ledger.id)}
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
