import { Card, Space, Typography } from "antd";
import type { MessageInstance } from "antd/es/message/interface";
import { useState } from "react";

import Timestamp from "../../../components/Timestamp";
import EditForm from "./Edit";
import { ILedger } from "../../../api/hyacinth";

interface IProps {
  item: ILedger;
  messageApi: MessageInstance;
  handleReload: () => void;
}

const { Meta } = Card;

const Widget = ({ item, messageApi, handleReload }: IProps) => {
  const [expanded, setExpanded] = useState(false);
  return (
    <Card
      title={item.label}
      extra={
        <EditForm
          item={item}
          messageApi={messageApi}
          handleReload={handleReload}
        />
      }
    >
      <Typography.Paragraph
        ellipsis={{
          rows: 4,
          expandable: "collapsible",
          expanded,
          onExpand: (_, info) => setExpanded(info.expanded),
        }}
        copyable
      >
        {item.memo}
      </Typography.Paragraph>
      <Meta
        title={item.uid}
        description={
          <Space>
            <Timestamp value={item.updatedAt} />
          </Space>
        }
      />
    </Card>
  );
};

export default Widget;
