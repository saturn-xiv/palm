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
  EmailFormSchema,
  type EmailFormData,
} from "../../../../api/portal";

interface IProps {
  submit: string;
  doSubmit: (email: string) => void;
}
const Widget = ({ submit, doSubmit }: IProps) => {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<EmailFormData>({
    resolver: zodResolver(EmailFormSchema),
  });

  const onSubmit = (data: EmailFormData) => {
    doSubmit(data.email);
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
      <Button type="submit" fullWidth variant="contained">
        <FormattedMessage id={submit} />
      </Button>
    </Box>
  );
};

export default Widget;
