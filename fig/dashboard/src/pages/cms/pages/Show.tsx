import { IPage } from "../../../api/carnation";

interface IProps {
  item: IPage;
}

const Widget = ({ item }: IProps) => {
  return <>page show({item.id})</>;
};

export default Widget;
