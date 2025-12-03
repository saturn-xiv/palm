import type { IInputRule } from "../../api/rules";

const sort_orders = (): number[] => {
  return [...Array(100).keys()].map((i) => i + 1000);
};

interface IProps {
  devices: string[];
  item?: IInputRule;
}
const Widget = ({ devices }: IProps) => {
  return <></>;
};

export default Widget;
