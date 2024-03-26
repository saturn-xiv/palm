import { useState } from "react";
import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";
import { FormattedMessage } from "react-intl";
import Typography from "@mui/material/Typography";
import { dinero, toDecimal } from "dinero.js";
import Autocomplete from "@mui/material/Autocomplete";
import FormControl from "@mui/material/FormControl";

import { ICurrencyOption } from "../api/camelia";

export interface IForm {
  amount: number;
  currency: string;
}

interface IProps {
  value?: IForm;
  currencies: ICurrencyOption[];
  handleChange: (amount: number, currency: string) => void;
}

const detect_base = (code: string): number => {
  switch (code) {
    default:
      return 10;
  }
};
const detect_exponent = (
  code: string,
  currencies: ICurrencyOption[]
): number => {
  const items = currencies.filter((x) => x.code === code);
  if (items.length > 0) {
    return items[0].unit;
  }
  return 2;
};

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
            const v = parseInt(e.target.value, 10);
            setItem({ amount: v, currency: item.currency });
            handleChange(v, item.currency);
          }}
          helperText={
            <Typography variant="caption" display="block" gutterBottom>
              {item.currency}: &nbsp;
              {toDecimal(
                dinero({
                  amount: Math.trunc(item.amount),
                  currency: {
                    code: item.currency,
                    base: detect_base(item.currency),
                    exponent: detect_exponent(item.currency, currencies),
                  },
                })
              )}
            </Typography>
          }
        />
        <Autocomplete
          disablePortal
          sx={{ width: 300 }}
          options={currencies}
          getOptionLabel={(x) => `${x.code}-${x.name}`}
          isOptionEqualToValue={(option, value): boolean => {
            return option.code === value.code;
          }}
          //   defaultValue={currencies.filter((x) => x.code === item.currency)[0]}
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
              value={item?.amount}
            />
          )}
        />
      </Stack>
    </FormControl>
  );
};

export default Widget;
