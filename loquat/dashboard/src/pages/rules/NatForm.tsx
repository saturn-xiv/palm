import type { INatRule } from "../../api/rules";

const sort_orders = (): number[] => {
  return [...Array(100).keys()].map((i) => i + 2000);
};

interface IProps {
  devices: string[];
  item?: INatRule;
}
const Widget = ({ devices }: IProps) => {
  return <></>;
};

export default Widget;
