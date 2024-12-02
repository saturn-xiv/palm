import { Space } from "antd";

import { IPostalRecipient } from "../../../api/daffodil";

interface IProps {
  item: IPostalRecipient;
}
const Widget = ({ item }: IProps) => {
  return <Space>{item.name}</Space>;
};

export default Widget;
