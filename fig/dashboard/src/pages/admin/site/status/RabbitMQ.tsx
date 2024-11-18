import { Card, Space, Typography } from "antd";

import { IRabbitMQStatus } from "../../../../api/daffodil";

const { Text } = Typography;

interface IProps {
  item?: IRabbitMQStatus;
}

const Widget = ({ item }: IProps) => {
  return item ? (
    <Card title="RabbitMQ">
      <Space>
        <Text>Username</Text>
        <Text code>{item.username}</Text>
      </Space>
      <br />
      <Space>
        <Text>Virtual host</Text>
        <Text code>{item.virtualHost}</Text>
      </Space>
    </Card>
  ) : (
    <></>
  );
};

export default Widget;
