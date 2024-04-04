import { dinero, toDecimal } from "dinero.js";

import { ICurrencyOption } from "../api/camelia";

export interface IProps {
  amount: number;
  currency: ICurrencyOption;
}

const Widget = ({ amount, currency }: IProps) => {
  return (
    <>
      {currency.code}:
      {toDecimal(
        dinero({
          amount: Math.trunc(amount),
          currency: { code: currency.code, base: 10, exponent: currency.unit },
        })
      )}
    </>
  );
};

export default Widget;
