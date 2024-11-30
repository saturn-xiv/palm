import { Button, Card, Space, Typography } from "antd";
import type { MessageInstance } from "antd/es/message/interface";
import { useState } from "react";
import { EyeOutlined } from "@ant-design/icons";
import { FormattedMessage } from "react-intl";

import Timestamp from "../../../components/Timestamp";
import EditForm from "./Edit";
import { ILedger } from "../../../api/hyacinth";
import { useNavigate } from "react-router-dom";

interface IProps {
  item: ILedger;
  messageApi: MessageInstance;
  handleReload: () => void;
}

const { Meta } = Card;

const Widget = ({ item, messageApi, handleReload }: IProps) => {
  const navigate = useNavigate();
  const [expanded, setExpanded] = useState(false);
  return (
    <Card
      title={item.label}
      extra={
        <Space>
          <Button
            onClick={() => {
              navigate(`/dashboard/accounting/ledgers/${item.id}`);
            }}
            icon={<EyeOutlined />}
            size="small"
          >
            <FormattedMessage id="buttons.view" />
          </Button>
          <EditForm
            item={item}
            messageApi={messageApi}
            handleReload={handleReload}
          />
        </Space>
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
