import type { IInputRule, INatRule, IPingRule, IRule } from "../../api/rules";
import ShowNat from "./ShowNat";
import ShowInput from "./ShowInput";
import ShowPing from "./ShowPing";

export const NAT_RULE = "Nat";
export const INPUT_RULE = "Input";
export const PING_RULE = "Ping";

interface IProps {
  item: IRule;
}

const Widget = ({ item }: IProps) => {
  if (item.__typename === NAT_RULE) {
    return <ShowNat item={item as INatRule} />;
  }
  if (item.__typename === PING_RULE) {
    return <ShowPing item={item as IPingRule} />;
  }
  if (item.__typename === INPUT_RULE) {
    return <ShowInput item={item as IInputRule} />;
  }
  return <></>;
};

export default Widget;
