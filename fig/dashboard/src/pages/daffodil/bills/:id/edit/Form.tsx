import { useState, useEffect } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import Autocomplete from "@mui/material/Autocomplete";
import { FormattedMessage, useIntl } from "react-intl";
import { string as yup_string, object as yup_object } from "yup";
import { useNavigate } from "react-router-dom";
import FormControl from "@mui/material/FormControl";
import Stack from "@mui/material/Stack";
import moment, { Moment } from "moment";
import { DateTimePicker } from "@mui/x-date-pickers/DateTimePicker";

import { IErrorMessage } from "../../../../../api/graphql";
import {
  bill_form_options,
  update_bill,
  IBill,
} from "../../../../../api/daffodil";
import { useAppDispatch } from "../../../../../hooks";
import { ICurrencyOption } from "../../../../../api/camelia";
import {
  success as success_box,
  error as error_box,
} from "../../../../../reducers/message-box";
import {
  DATETIME_PICKER_FORMAT,
  LOCAL_DATETIME_FORMAT,
} from "../../../../../components";
import AmountInput, {
  IForm as IAmount,
} from "../../../../../components/AmountInput";

const validationSchema = yup_object({
  summary: yup_string().required().min(1).max(511),
  merchant: yup_string().required().min(1).max(63),
  category: yup_string().required().min(1).max(31),
  paidBy: yup_string().required().min(1).max(31),
  reason: yup_string().required().min(1).max(127),
});

interface IProps {
  item: IBill;
}

function Widget({ item }: IProps) {
  const navigate = useNavigate();
  const dispatch = useAppDispatch();
  const intl = useIntl();
  const [paidAt, setPaidAt] = useState<Moment>(moment(item.paidAt));
  const [paymentMethods, setPaymentMethods] = useState<string[]>([]);
  const [merchants, setMerchants] = useState<string[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [currencies, setCurrencies] = useState<ICurrencyOption[]>([]);
  const [amount, setAmount] = useState<IAmount>({
    amount: item.amount,
    currency: item.currency.code,
  });

  useEffect(() => {
    bill_form_options().then((res) => {
      setCurrencies(res.currencies);
      setMerchants(res.merchants);
      setPaymentMethods(res.payment_methods);
      setCategories(res.categories);
    });
  }, []);

  const formik = useFormik({
    enableReinitialize: true,
    initialValues: {
      summary: item.summary,
      merchant: item.merchant,
      category: item.category,
      paidBy: item.paidBy,
      reason: "",
    },
    validationSchema,
    onSubmit: (values) => {
      //   console.log(paidAt.format(DATETIME_PICKER_FORMAT), amount, values);

      update_bill(
        item.id,
        values.summary,
        amount?.amount,
        amount?.currency,
        values.merchant,
        values.category,
        paidAt.format(LOCAL_DATETIME_FORMAT),
        values.paidBy,
        values.reason
      )
        .then(() => {
          dispatch(
            success_box([intl.formatMessage({ id: "flashes.succeed" })])
          );
          navigate(`/dashboard/daffodil/bills/${item.id}`);
        })
        .catch((reason: IErrorMessage[]) => {
          dispatch(error_box(reason.map((x) => x.message)));
        });
    },
  });

  return currencies.length > 0 ? (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage
          id="daffodil.bills.edit.title"
          values={{ id: item.id }}
        />
      </Typography>
      <Box
        component="form"
        onSubmit={formik.handleSubmit}
        noValidate
        sx={{ mt: 1 }}
      >
        <AmountInput
          value={amount}
          handleChange={(a, c) => {
            setAmount({ amount: a, currency: c });
          }}
          currencies={currencies}
        />

        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.summary.label" })}
          name="summary"
          multiline
          rows={4}
          value={formik.values.summary}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.summary && Boolean(formik.errors.summary)}
          helperText={formik.touched.summary && formik.errors.summary}
        />

        <Autocomplete
          disablePortal
          freeSolo
          value={formik.values.merchant}
          options={merchants.map((x) => {
            return { label: x };
          })}
          renderInput={(params) => (
            <TextField
              {...params}
              margin="normal"
              required
              fullWidth
              label={intl.formatMessage({ id: "form.fields.merchant.label" })}
              name="merchant"
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.merchant && Boolean(formik.errors.merchant)}
              helperText={formik.touched.merchant && formik.errors.merchant}
            />
          )}
        />
        <Autocomplete
          disablePortal
          freeSolo
          value={formik.values.category}
          options={categories.map((x) => {
            return { label: x };
          })}
          renderInput={(params) => (
            <TextField
              {...params}
              margin="normal"
              required
              fullWidth
              label={intl.formatMessage({ id: "form.fields.category.label" })}
              name="category"
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.category && Boolean(formik.errors.category)}
              helperText={formik.touched.category && formik.errors.category}
            />
          )}
        />
        <FormControl margin="normal" required>
          <DateTimePicker<Moment>
            label={<FormattedMessage id="form.fields.paid-at.label" />}
            name="paidAt"
            value={paidAt}
            onChange={(v) => {
              if (v) {
                setPaidAt(v);
              }
            }}
            format={DATETIME_PICKER_FORMAT}
          />
        </FormControl>
        <Autocomplete
          disablePortal
          freeSolo
          value={formik.values.paidBy}
          options={paymentMethods.map((x) => {
            return { label: x };
          })}
          renderInput={(params) => (
            <TextField
              {...params}
              margin="normal"
              required
              fullWidth
              label={intl.formatMessage({ id: "form.fields.paid-by.label" })}
              name="paidBy"
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.paidBy && Boolean(formik.errors.paidBy)}
              helperText={formik.touched.paidBy && formik.errors.paidBy}
            />
          )}
        />

        <TextField
          margin="normal"
          required
          fullWidth
          label={intl.formatMessage({ id: "form.fields.reason.label" })}
          name="reason"
          multiline
          rows={2}
          value={formik.values.reason}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.reason && Boolean(formik.errors.reason)}
          helperText={formik.touched.reason && formik.errors.reason}
        />

        <Stack spacing={2} direction="row" justifyContent="flex-end">
          <Button
            variant="contained"
            color="inherit"
            onClick={() => {
              navigate(`/dashboard/daffodil/bills/${item.id}`);
            }}
          >
            <FormattedMessage id="buttons.go-back" />
          </Button>
          <Button variant="contained" type="submit">
            <FormattedMessage id="buttons.submit" />
          </Button>
        </Stack>
      </Box>
    </>
  ) : (
    <></>
  );
}

export default Widget;
