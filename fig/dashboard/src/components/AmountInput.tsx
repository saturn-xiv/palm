import { useState } from "react";
import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";
import { FormattedMessage } from "react-intl";
import Typography from "@mui/material/Typography";
import Autocomplete from "@mui/material/Autocomplete";
import FormControl from "@mui/material/FormControl";

import { ICurrencyOption } from "../api/camelia";
import AmountShow from "./AmountShow";

export interface IForm {
  amount: number;
  currency: string;
}

interface IProps {
  value?: IForm;
  currencies: ICurrencyOption[];
  handleChange: (amount: number, currency: string) => void;
}

const Widget = ({ value, currencies, handleChange }: IProps) => {
  const [item, setItem] = useState<IForm>({
    amount: value?.amount || 0,
    currency: value?.currency || import.meta.env.VITE_DEFAULT_CURRENCY,
  });

  return (
    <FormControl margin="normal" required fullWidth>
      <Stack spacing={2} direction="row">
        <TextField
          required
          label={<FormattedMessage id="form.fields.amount.label" />}
          type="number"
          value={item.amount}
          onChange={(e) => {
            if (e.target.value) {
              const v = parseInt(e.target.value, 10);
              setItem({ amount: v, currency: item.currency });
              handleChange(v, item.currency);
            }
          }}
          helperText={
            <Typography variant="caption" display="block" gutterBottom>
              <AmountShow
                amount={item.amount}
                currency={currencies.filter((x) => x.code === item.currency)[0]}
              />
            </Typography>
          }
        />
        <Autocomplete
          disablePortal
          sx={{ width: 300 }}
          options={currencies}
          value={currencies.filter((x) => x.code === item.currency)[0]}
          getOptionLabel={(x) => `${x.code}-${x.name}`}
          isOptionEqualToValue={(option, value): boolean => {
            return option.code === value.code;
          }}
          onChange={(_e, v) => {
            if (v !== null) {
              setItem({ amount: item.amount, currency: v.code });
              handleChange(item.amount, v.code);
            }
          }}
          renderInput={(params) => (
            <TextField
              required
              {...params}
              label={<FormattedMessage id="form.fields.currency.label" />}
            />
          )}
        />
      </Stack>
    </FormControl>
  );
};

export default Widget;
