import { Card, List } from "antd";

import { IPostgreSqlStatus } from "../../../../api/camelia";

interface IProps {
  item: IPostgreSqlStatus;
}

const Widget = ({ item }: IProps) => {
  return (
    <Card title="PostgreSql" hoverable>
      <List<string>
        bordered
        header="Migrations"
        dataSource={item.migrations}
        renderItem={(item) => <List.Item>{item}</List.Item>}
      />
      <br />
      <Card.Meta description={item.version} />
    </Card>
  );
};

export default Widget;
