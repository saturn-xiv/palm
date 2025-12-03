import type { IInputRule } from "../../api/rules";
import { protocol } from "../../components";

interface IProps {
  item: IInputRule;
}

const Widget = ({ item }: IProps) => {
  return (
    <span>
      {item.__typename} {protocol(item.tcp)}:{item.port}@{item.device}
    </span>
  );
};

export default Widget;
