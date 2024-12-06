import { Button, Space, Tooltip, Typography } from "antd";
import { useState } from "react";
import { EyeOutlined } from "@ant-design/icons";

import { IAttachment, show_attachment } from "../../api/daffodil";
import { FormattedMessage } from "react-intl";

interface IProps {
  item: IAttachment;
}
const Widget = ({ item }: IProps) => {
  const [url, setUrl] = useState<string | undefined>();

  return (
    <Space>
      <Button
        type="text"
        size="small"
        onClick={() => {
          show_attachment(item.id).then((res) => {
            setUrl(res.url);
          });
        }}
      >
        {item.title}
      </Button>
      {url && (
        <Tooltip title={<FormattedMessage id="buttons.open" />}>
          <Button
            size="small"
            icon={<EyeOutlined />}
            onClick={() => {
              window.open(url, "_blank")?.focus();
            }}
          />
        </Tooltip>
      )}

      {url && <Typography.Text copyable={{ text: url }} />}
    </Space>
  );
};

export default Widget;
