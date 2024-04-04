import { useState } from "react";
import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";
import { FormattedMessage } from "react-intl";
import Typography from "@mui/material/Typography";
import Autocomplete from "@mui/material/Autocomplete";
import Radio from "@mui/material/Radio";
import RadioGroup from "@mui/material/RadioGroup";
import FormControlLabel from "@mui/material/FormControlLabel";
import FormControl from "@mui/material/FormControl";
import FormLabel from "@mui/material/FormLabel";

import { ICurrencyOption } from "../api/camelia";
import AmountShow from "./AmountShow";

export const INCOME = "income";
export const EXPENSE = "expense";

export interface IForm {
  type: string;
  amount: number;
  currency: string;
}

interface IProps {
  value: IForm;
  currencies: ICurrencyOption[];
  handleChange: (amount: number, currency: string, type: string) => void;
}

const Widget = ({ value, currencies, handleChange }: IProps) => {
  const [item, setItem] = useState<IForm>({
    type: value.type,
    amount: value.amount,
    currency: value.currency,
  });

  return (
    <>
      <FormControl margin="normal" required fullWidth>
        <FormLabel id="amount-form-type-label">
          <FormattedMessage id="form.fields.type.label" />
        </FormLabel>
        <RadioGroup
          row
          aria-labelledby="amount-form-type-label"
          name="type"
          value={item.type}
          onChange={(_e, v) => {
            setItem({
              amount: item.amount,
              currency: item.currency,
              type: v,
            });
            handleChange(item.amount, item.currency, v);
          }}
        >
          <FormControlLabel
            value={INCOME}
            control={<Radio />}
            label={<FormattedMessage id="form.fields.income.label" />}
          />
          <FormControlLabel
            value={EXPENSE}
            control={<Radio />}
            label={<FormattedMessage id="form.fields.expense.label" />}
          />
        </RadioGroup>
      </FormControl>
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
                setItem({
                  amount: v,
                  currency: item.currency,
                  type: item.type,
                });
                handleChange(v, item.currency, item.type);
              }
            }}
            helperText={
              <Typography variant="caption" display="block" gutterBottom>
                <AmountShow
                  amount={item.amount}
                  currency={
                    currencies.filter((x) => x.code === item.currency)[0]
                  }
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
                setItem({
                  amount: item.amount,
                  currency: v.code,
                  type: item.type,
                });
                handleChange(item.amount, v.code, item.type);
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
    </>
  );
};

export default Widget;
