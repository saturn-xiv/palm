import { Card, Space, Typography } from "antd";

import { IPostgreSqlStatus } from "../../../../api/daffodil";
import Timestamp from "../../../../components/Timestamp";

const { Text } = Typography;

interface IProps {
  item?: IPostgreSqlStatus;
}

const Widget = ({ item }: IProps) => {
  return item ? (
    <Card title="PostgreSql">
      <Space>
        <Text>Timestamp</Text>
        <Text code>
          <Timestamp value={item.timestamp} />
        </Text>
      </Space>
      <br />
      <Space>
        <Text>Version</Text>
        <Text code>{item.version}</Text>
      </Space>
    </Card>
  ) : (
    <></>
  );
};

export default Widget;
