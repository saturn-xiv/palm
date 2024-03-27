import { FormattedMessage } from "react-intl";
import Typography from "@mui/material/Typography";

import Amount from "../../../../../components/AmountShow";
import { ICurrencyOption } from "../../../../../api/camelia";

interface IProps {
  amount: number;
  currency: ICurrencyOption;
  abbreviation?: boolean;
}

const Widget = ({ amount, currency, abbreviation }: IProps) => {
  return (
    <Typography variant="button" display="block" gutterBottom>
      <Amount amount={amount} currency={currency} />
      (
      <FormattedMessage
        id={`daffodil.fields.${amount > 0 ? "income" : "expense"}.${
          abbreviation ? "abbreviation" : "label"
        }`}
      />
      )
    </Typography>
  );
};

export default Widget;
