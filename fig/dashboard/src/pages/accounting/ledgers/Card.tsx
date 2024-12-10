import { Button, Card, Space, Typography } from "antd";
import type { MessageInstance } from "antd/es/message/interface";
import { useState } from "react";
import { EyeOutlined } from "@ant-design/icons";
import { FormattedMessage } from "react-intl";
import { useNavigate } from "react-router-dom";

import Timestamp from "../../../components/Timestamp";
import EditForm from "./Edit";
import SetCover from "./SetCover";
import { ILedger } from "../../../api/hyacinth";
import Cover from "../../attachments/Cover";

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
      hoverable
      title={item.label}
      cover={<Cover items={item.covers} />}
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
          <SetCover item={item} />
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
