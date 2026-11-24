import Button from "@mui/material/Button";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { FormattedMessage, useIntl } from "react-intl";

import {
  PASSWORD_PLACEHOLDER,
  SignUpBySmsFormSchema,
  type SignUpBySmsFormData,
} from "../../../../api/portal";
import { useEffect, useState } from "react";
import {
  DEFAULT_COUNTRY_DIAL_REGION,
  indexCountryDialCodes,
  type ICountryDialCode,
} from "../../../../api/portal/phone-number";
import { useAppSelector } from "../../../../hooks";
import { selectLayout } from "../../../../reducers/layout";

const Widget = () => {
  const site = useAppSelector(selectLayout);
  const [countryDialCodes, setCountryDialCodes] = useState<ICountryDialCode[]>([
    { code: 0, name: "", region: DEFAULT_COUNTRY_DIAL_REGION },
  ]);
  const intl = useIntl();
  const {
    register,
    handleSubmit,
    getValues,
    formState: { errors },
  } = useForm<SignUpBySmsFormData>({
    resolver: zodResolver(SignUpBySmsFormSchema),
    defaultValues: { countryDialCode: DEFAULT_COUNTRY_DIAL_REGION },
  });

  const onSubmit = (data: SignUpBySmsFormData) => {
    // TODO
    console.log(data);
  };
  const onFetchVerifyCode = () => {
    const data = getValues();
    console.log(data);
  };

  useEffect(() => {
    const call = async () => {
      if (site?.locale) {
        const res = await indexCountryDialCodes(site?.locale);
        setCountryDialCodes(res);
      }
    };
    call();
  }, [site]);

  return (
    <Box
      component="form"
      onSubmit={handleSubmit(onSubmit)}
      noValidate
      sx={{ display: "flex", flexDirection: "column", width: "100%", gap: 2 }}
    >
      <FormControl>
        <FormLabel htmlFor="name">
          <FormattedMessage id="fields.username.label" />
        </FormLabel>
        <TextField
          {...register("name")}
          error={errors.name !== undefined}
          helperText={errors.name?.message || ""}
          placeholder={intl.formatMessage({
            id: "fields.username.placeholder",
          })}
          autoComplete="username"
          fullWidth
          variant="outlined"
          color={errors.name ? "error" : "primary"}
        />
      </FormControl>

      <FormControl>
        <FormLabel htmlFor="phoneNumber">
          <FormattedMessage id="fields.phone-number.label" />
        </FormLabel>
        <Stack direction="row" spacing={2}>
          <Select
            {...register("countryDialCode")}
            label={intl.formatMessage({ id: "fields.country-dial-code.label" })}
            error={errors.countryDialCode !== undefined}
            color={errors.countryDialCode ? "error" : "primary"}
            defaultValue={DEFAULT_COUNTRY_DIAL_REGION}
          >
            {countryDialCodes.map((x, i) => (
              <MenuItem key={i} value={x.region}>
                +{x.code}&nbsp;{x.name}
              </MenuItem>
            ))}
          </Select>
          <TextField
            {...register("phoneNumber")}
            error={errors.phoneNumber !== undefined}
            helperText={errors.phoneNumber?.message || ""}
            placeholder={intl.formatMessage({
              id: "fields.phone-number.placeholder",
            })}
            autoComplete="phone-number"
            variant="outlined"
            color={errors.phoneNumber ? "error" : "primary"}
          />
        </Stack>
      </FormControl>
      <FormControl>
        <FormLabel htmlFor="verifyCode">
          <FormattedMessage id="fields.verify-code.label" />
        </FormLabel>
        <Stack direction="row" spacing={2}>
          <TextField
            {...register("verifyCode")}
            error={errors.verifyCode !== undefined}
            helperText={errors.verifyCode?.message || ""}
            variant="outlined"
            color={errors.verifyCode ? "error" : "primary"}
            sx={{ width: "12ch" }}
          />
          <Button onClick={onFetchVerifyCode} variant="text" size="small">
            <FormattedMessage id="fields.verify-code.fetch" />
          </Button>
        </Stack>
      </FormControl>
      <FormControl>
        <Box sx={{ display: "flex", justifyContent: "space-between" }}>
          <FormLabel htmlFor="password">
            <FormattedMessage id="fields.password.label" />
          </FormLabel>
        </Box>
        <TextField
          {...register("password")}
          error={errors.password !== undefined}
          helperText={errors.password?.message || ""}
          placeholder={PASSWORD_PLACEHOLDER}
          type="password"
          autoComplete="current-password"
          fullWidth
          variant="outlined"
          color={errors.password ? "error" : "primary"}
        />
      </FormControl>
      <FormControl>
        <Box sx={{ display: "flex", justifyContent: "space-between" }}>
          <FormLabel htmlFor="passwordConfirmation">
            <FormattedMessage id="fields.password-confirmation.label" />
          </FormLabel>
        </Box>
        <TextField
          {...register("passwordConfirmation")}
          error={errors.passwordConfirmation !== undefined}
          helperText={errors.passwordConfirmation?.message || ""}
          placeholder={PASSWORD_PLACEHOLDER}
          type="password"
          fullWidth
          variant="outlined"
          color={errors.passwordConfirmation ? "error" : "primary"}
        />
      </FormControl>
      <Button type="submit" fullWidth variant="contained">
        <FormattedMessage id="portal.pages.anonymous.users.sign-up.request" />
      </Button>
    </Box>
  );
};

export default Widget;
