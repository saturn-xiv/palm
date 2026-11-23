import Button from "@mui/material/Button";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { FormattedMessage } from "react-intl";

import {
  EMAIL_PLACEHOLDER,
  PASSWORD_PLACEHOLDER,
  SignInByEmailFormSchema,
  type SignInByEmailFormData,
} from "../../../../api/portal";

const Widget = () => {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<SignInByEmailFormData>({
    resolver: zodResolver(SignInByEmailFormSchema),
  });

  const onSubmit = (data: SignInByEmailFormData) => {
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
        <FormLabel htmlFor="email">
          <FormattedMessage id="fields.email.label" />
        </FormLabel>
        <TextField
          {...register("email")}
          error={errors.email !== undefined}
          helperText={errors.email?.message || ""}
          placeholder={EMAIL_PLACEHOLDER}
          autoComplete="email"
          fullWidth
          variant="outlined"
          color={errors.email ? "error" : "primary"}
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
