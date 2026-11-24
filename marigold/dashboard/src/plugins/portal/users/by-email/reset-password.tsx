import { useParams } from "react-router";
import Button from "@mui/material/Button";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { FormattedMessage, useIntl } from "react-intl";

import {
  PASSWORD_PLACEHOLDER,
  ResetPasswordForEmailUserFormSchema,
  type ResetPasswordForEmailUserFormData,
} from "../../../../api/portal";
import Title from "../../../../layouts/Title";
import SharedLinksByEmail from "../SharedLinks";

const Widget = () => {
  const { token } = useParams();
  const intl = useIntl();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<ResetPasswordForEmailUserFormData>({
    resolver: zodResolver(ResetPasswordForEmailUserFormSchema),
  });

  const onSubmit = (data: ResetPasswordForEmailUserFormData) => {
    // TODO
    console.log(data, token);
  };

  return (
    <>
      <Typography
        component="h1"
        variant="h4"
        sx={{ width: "100%", fontSize: "clamp(2rem, 10vw, 2.15rem)" }}
      >
        <Title
          value={intl.formatMessage({
            id: "portal.pages.anonymous.users.by-email.reset-password.title",
          })}
        />
      </Typography>
      <Box
        component="form"
        onSubmit={handleSubmit(onSubmit)}
        noValidate
        sx={{ display: "flex", flexDirection: "column", width: "100%", gap: 2 }}
      >
        <FormControl>
          <Box sx={{ display: "flex", justifyContent: "space-between" }}>
            <FormLabel htmlFor="password">
              <FormattedMessage id="fields.new-password.label" />
            </FormLabel>
          </Box>
          <TextField
            {...register("newPassword")}
            error={errors.newPassword !== undefined}
            helperText={errors.newPassword?.message || ""}
            placeholder={PASSWORD_PLACEHOLDER}
            type="password"
            autoComplete="current-password"
            fullWidth
            variant="outlined"
            color={errors.newPassword ? "error" : "primary"}
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
          <FormattedMessage id="buttons.submit" />
        </Button>
      </Box>
      <SharedLinksByEmail />
    </>
  );
};

export default Widget;
