import { useState, useEffect } from "react";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import { useFormik } from "formik";
import TextField from "@mui/material/TextField";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import InputLabel from "@mui/material/InputLabel";
import Autocomplete from "@mui/material/Autocomplete";
import { FormattedMessage, useIntl } from "react-intl";
import {
  string as yup_string,
  number as yup_number,
  object as yup_object,
} from "yup";
import { useNavigate } from "react-router-dom";
import FormControl from "@mui/material/FormControl";
import Stack from "@mui/material/Stack";
import moment, { Moment } from "moment";
import { DateTimePicker } from "@mui/x-date-pickers/DateTimePicker";

import { IErrorMessage } from "../../../../../api/graphql";
import { create_bill, ILedger } from "../../../../../api/daffodil";
import { useAppDispatch } from "../../../../../hooks";
import { currency_options, ICurrencyOption } from "../../../../../api/camelia";
import {
  success as success_box,
  error as error_box,
} from "../../../../../reducers/message-box";
import Upload from "../../../../attachments/Upload";
import { DATETIME_PICKER_FORMAT } from "../../../../../components";

const validationSchema = yup_object({
  summary: yup_string().required().min(1).max(511),
  price: yup_number().required(),
  currency: yup_string().required().length(3),
  merchant: yup_string().required().min(1).max(63),
  category: yup_string().required().min(1).max(31),
  paidBy: yup_string().required().min(1).max(31),
});

interface IProps {
  item: ILedger;
}

function Widget({ item }: IProps) {
  const navigate = useNavigate();
  const dispatch = useAppDispatch();
  const intl = useIntl();
  const [paidAt, setPaidAt] = useState<Moment>(moment());
  const [paymentMethods, setPaymentMethods] = useState<string[]>([]);
  const [merchants, setMerchants] = useState<string[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [currencies, setCurrencies] = useState<ICurrencyOption[]>([]);

  useEffect(() => {
    setPaymentMethods(["p1", "p2"]);
    setMerchants(["m1", "m2"]);
    setCategories(["c1", "c2"]);
    currency_options().then((res) => {
      setCurrencies(res);
    });
  }, []);

  const formik = useFormik({
    initialValues: {
      summary: "",
      price: 0.0,
      currency: "USD",
      merchant: "",
      category: "",
      paidBy: "",
    },
    validationSchema,
    onSubmit: (values) => {
      console.log(paidAt.format(DATETIME_PICKER_FORMAT), values);
      return;
      create_bill(
        item.id,
        values.summary,
        values.price,
        values.currency,
        values.merchant,
        values.category,
        paidAt.format(DATETIME_PICKER_FORMAT),
        values.paidBy
      )
        .then(() => {
          dispatch(
            success_box([intl.formatMessage({ id: "flashes.succeed" })])
          );
          navigate(`/dashboard/daffodil/ledgers/${item.id}`);
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
          id="daffodil.ledgers.append-bill.title"
          values={{ name: item.name }}
        />
      </Typography>
      <Box
        component="form"
        onSubmit={formik.handleSubmit}
        noValidate
        sx={{ mt: 1 }}
      >
        <TextField
          margin="normal"
          required
          label={intl.formatMessage({ id: "form.fields.price.label" })}
          name="price"
          type="number"
          value={formik.values.price}
          onChange={formik.handleChange}
          onBlur={formik.handleBlur}
          error={formik.touched.price && Boolean(formik.errors.price)}
          helperText={formik.touched.price && formik.errors.price}
        />
        <FormControl margin="normal" required fullWidth>
          <InputLabel id="daffodil-append-bill-currency-select-label">
            <FormattedMessage id="form.fields.currency.label" />
          </InputLabel>
          <Select
            labelId="daffodil-append-bill-currency-select-label"
            name="currency"
            value={formik.values.currency}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          >
            {currencies.map((x, i) => (
              <MenuItem key={i} value={x.code}>
                {x.name}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
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
              value={formik.values.merchant}
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
              value={formik.values.category}
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
              value={formik.values.paidBy}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.paidBy && Boolean(formik.errors.paidBy)}
              helperText={formik.touched.paidBy && formik.errors.paidBy}
            />
          )}
        />

        <Upload
          accept={{
            "image/png": [".png"],
            "image/jpeg": [".jpg", ".jpeg"],
            "application/pdf": [".pdf"],
          }}
          handleRefresh={() => {
            // TODO
          }}
        />

        <Stack spacing={2} direction="row" justifyContent="flex-end">
          <Button
            variant="contained"
            color="inherit"
            onClick={() => {
              navigate(`/dashboard/daffodil/ledgers/${item.id}`);
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
