import { List, Typography } from "antd";

import { IRedisStatus } from "../../../../api/daffodil";

interface IProps {
  item?: IRedisStatus;
}

const Widget = ({ item }: IProps) => {
  return item ? (
    <List
      size="small"
      header={<Typography.Title level={4}>Redis</Typography.Title>}
      bordered
      dataSource={item.version}
      renderItem={(x, i) => <List.Item key={i}>{x}</List.Item>}
      pagination={{}}
    />
  ) : (
    <></>
  );
};

export default Widget;
