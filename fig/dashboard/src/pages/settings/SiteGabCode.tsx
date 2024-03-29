import { IGabCode } from "../../api/camelia";

interface IProps {
  item?: IGabCode;
  handleRefresh: () => void;
}
const Widget = ({ item }: IProps) => {
  return <>gab code</>;
};
export default Widget;
