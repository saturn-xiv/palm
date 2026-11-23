import Button from "@mui/material/Button";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { FormattedMessage, useIntl } from "react-intl";

import {
  PASSWORD_PLACEHOLDER,
  SignInBySmsFormSchema,
  type SignInBySmsFormData,
} from "../../../../api/portal";

const Widget = () => {
  const intl = useIntl();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<SignInBySmsFormData>({
    resolver: zodResolver(SignInBySmsFormSchema),
  });

  const onSubmit = (data: SignInBySmsFormData) => {
    // TODO
    console.log(data);
  };

  return (
    <Box
      component="form"
      onSubmit={handleSubmit(onSubmit)}
      noValidate
      sx={{ display: "flex", flexDirection: "column", width: "100%", gap: 2 }}
    >
      <FormControl>
        <FormLabel htmlFor="phoneNumber">
          <FormattedMessage id="fields.phone-number.label" />
        </FormLabel>
        <TextField
          {...register("phoneNumber")}
          error={errors.phoneNumber !== undefined}
          helperText={errors.phoneNumber?.message || ""}
          placeholder={intl.formatMessage({
            id: "fields.phone-number.placeholder",
          })}
          autoComplete="phone-number"
          fullWidth
          variant="outlined"
          color={errors.phoneNumber ? "error" : "primary"}
        />
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
      <Button type="submit" fullWidth variant="contained">
        <FormattedMessage id="portal.pages.anonymous.users.sign-up.request" />
      </Button>
    </Box>
  );
};

export default Widget;
