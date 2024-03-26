import { dinero, toDecimal } from "dinero.js";
import Typography from "@mui/material/Typography";

import { ICurrencyOption } from "../api/camelia";

interface IProps {
  value: number;
  currency: ICurrencyOption;
}
const Widget = ({ value, currency }: IProps) => {
  return (
    <Typography variant="caption" display="block" gutterBottom>
      {currency.name}: &nbsp;
      {toDecimal(
        dinero({
          amount: Math.trunc(value),
          currency: { code: currency.code, base: 10, exponent: currency.unit },
        })
      )}
    </Typography>
  );
};

export default Widget;
