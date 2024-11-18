import { List, Typography } from "antd";

import { IMinioStatus } from "../../../../api/daffodil";

interface IProps {
  item?: IMinioStatus;
}

const Widget = ({ item }: IProps) => {
  return item ? (
    <List
      size="small"
      header={<Typography.Title level={4}>Minio</Typography.Title>}
      bordered
      dataSource={item.buckets}
      renderItem={(x, i) => <List.Item key={i}>{x}</List.Item>}
      pagination={{}}
    />
  ) : (
    <></>
  );
};

export default Widget;
