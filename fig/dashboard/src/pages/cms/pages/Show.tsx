import { IPage } from "../../../api/cms";

interface IProps {
  item: IPage;
}

const Widget = ({ item }: IProps) => {
  return <>page show({item.id})</>;
};

export default Widget;
