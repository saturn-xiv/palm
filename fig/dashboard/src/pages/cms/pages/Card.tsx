import { Card } from "antd";

import { IPage } from "../../../api/cms";

interface IProps {
  item: IPage;
}

const Widget = ({ item }: IProps) => {
  return (
    <Card
      title={item.title}
      extra={<a href="#">More</a>}
      style={{ width: 300 }}
    >
      <p>{item.body}</p>
    </Card>
  );
};

export default Widget;
