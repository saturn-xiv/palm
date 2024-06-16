import { Card, List } from "antd";

import { IRedisStatus } from "../../../../api/site";

interface IProps {
  item: IRedisStatus;
}

const Widget = ({ item }: IProps) => {
  return (
    <Card title="Redis" hoverable>
      <List<string>
        bordered
        header="Nodes"
        dataSource={item.nodes}
        renderItem={(item) => <List.Item>{item}</List.Item>}
      />
    </Card>
  );
};

export default Widget;
