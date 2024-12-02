import { Space } from "antd";

import { IPostalAddress } from "../../../api/daffodil";

interface IProps {
  item: IPostalAddress;
}
const Widget = ({ item }: IProps) => {
  return (
    <Space>
      {item.street}
      {item.city}
      {item.province}
      {item.country}
      {item.zipCode}
    </Space>
  );
};

export default Widget;
