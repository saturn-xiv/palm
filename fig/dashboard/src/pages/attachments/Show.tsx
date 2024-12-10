import { Button, Space, Tooltip, Typography } from "antd";
import { EyeOutlined } from "@ant-design/icons";
import { FormattedMessage } from "react-intl";

import { IAttachment } from "../../api/daffodil";

interface IProps {
  item: IAttachment;
}
const Widget = ({ item }: IProps) => {
  return (
    <Space>
      {item.title}
      <Tooltip title={<FormattedMessage id="buttons.show" />}>
        <Button
          size="small"
          icon={<EyeOutlined />}
          onClick={() => {
            window.open(item.url, "_blank")?.focus();
          }}
        />
      </Tooltip>
      <Typography.Text copyable={{ text: item.url }} />
    </Space>
  );
};

export default Widget;
