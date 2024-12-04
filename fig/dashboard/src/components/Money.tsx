import { from_cents } from ".";
import { ICurrency } from "../api/daffodil";

interface IProps {
  currency: ICurrency;
  amount: number;
}

const Widget = ({ currency, amount }: IProps) => {
  return (
    <>
      {currency.code}: {from_cents(currency, amount)}
    </>
  );
};

export default Widget;
