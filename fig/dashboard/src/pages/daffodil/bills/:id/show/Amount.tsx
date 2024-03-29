import { FormattedMessage } from "react-intl";
import Typography from "@mui/material/Typography";

import Amount from "../../../../../components/AmountShow";
import { ICurrencyOption } from "../../../../../api/camelia";
import { EXPENSE, INCOME } from "../../../../../components/AmountInput";

interface IProps {
  amount: number;
  currency: ICurrencyOption;
  abbreviation?: boolean;
}

const Widget = ({ amount, currency, abbreviation }: IProps) => {
  return (
    <Typography
      variant="button"
      display="block"
      gutterBottom
      style={{
        color: amount >= 0 ? "cornflowerblue" : "brown",
      }}
    >
      <Amount amount={amount} currency={currency} />
      (
      <FormattedMessage
        id={`form.fields.${amount > 0 ? INCOME : EXPENSE}.${
          abbreviation ? "abbreviation" : "label"
        }`}
      />
      )
    </Typography>
  );
};

export default Widget;
