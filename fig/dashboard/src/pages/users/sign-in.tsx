import { useState } from "react";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Checkbox from "@mui/material/Checkbox";
import FormLabel from "@mui/material/FormLabel";
import FormControl from "@mui/material/FormControl";
import FormControlLabel from "@mui/material/FormControlLabel";
import Link from "@mui/material/Link";
import TextField from "@mui/material/TextField";
import Typography from "@mui/material/Typography";
import { FormattedMessage, useIntl } from "react-intl";

import Layout from "../../layouts/sign-in-side/Card";

const Widget = () => {
  const intl = useIntl();
  const [emailError, setEmailError] = useState(false);
  const [emailErrorMessage, setEmailErrorMessage] = useState("");
  const [passwordError, setPasswordError] = useState(false);
  const [passwordErrorMessage, setPasswordErrorMessage] = useState("");

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    if (emailError || passwordError) {
      event.preventDefault();
      return;
    }
    const data = new FormData(event.currentTarget);
    console.log({
      email: data.get("email"),
      password: data.get("password"),
    });
  };

  const validateInputs = () => {
    const email = document.getElementById("email") as HTMLInputElement;
    const password = document.getElementById("password") as HTMLInputElement;

    let isValid = true;

    if (!email.value || !/\S+@\S+\.\S+/.test(email.value)) {
      setEmailError(true);
      setEmailErrorMessage("Please enter a valid email address.");
      isValid = false;
    } else {
      setEmailError(false);
      setEmailErrorMessage("");
    }

    if (!password.value || password.value.length < 6) {
      setPasswordError(true);
      setPasswordErrorMessage("Password must be at least 6 characters long.");
      isValid = false;
    } else {
      setPasswordError(false);
      setPasswordErrorMessage("");
    }

    return isValid;
  };
  return (
    <Layout
      title={intl.formatMessage({ id: "pages.users.sign-in.title" })}
      handleSubmit={handleSubmit}
    >
      <FormControl>
        <FormLabel htmlFor="user">
          <FormattedMessage id="form.fields.email-or-nickname.label" />
        </FormLabel>
        <TextField
          error={emailError}
          helperText={emailErrorMessage}
          id="user"
          type="text"
          name="user"
          placeholder="Who am I?"
          autoComplete="user"
          autoFocus
          required
          fullWidth
          variant="outlined"
          color={emailError ? "error" : "primary"}
          sx={{ ariaLabel: "user" }}
        />
      </FormControl>
      <FormControl>
        <Box sx={{ display: "flex", justifyContent: "space-between" }}>
          <FormLabel htmlFor="password">
            <FormattedMessage id="form.fields.password.label" />
          </FormLabel>
          <Link
            component="button"
            type="button"
            variant="body2"
            sx={{ alignSelf: "baseline" }}
          >
            Forgot your password?
          </Link>
        </Box>
        <TextField
          error={passwordError}
          helperText={passwordErrorMessage}
          name="password"
          placeholder="••••••"
          type="password"
          id="password"
          autoComplete="current-password"
          autoFocus
          required
          fullWidth
          variant="outlined"
          color={passwordError ? "error" : "primary"}
        />
      </FormControl>
      <FormControlLabel
        control={<Checkbox value="remember" color="primary" />}
        label="Remember me"
      />
      <Button
        type="submit"
        fullWidth
        variant="contained"
        onClick={validateInputs}
      >
        <FormattedMessage id="buttons.submit" />
      </Button>
      <Typography sx={{ textAlign: "center" }}>
        Don&apos;t have an account? &nbsp;
        <span>
          <Link
            href="/material-ui/getting-started/templates/sign-in/"
            variant="body2"
            sx={{ alignSelf: "center" }}
          >
            Sign up
          </Link>
        </span>
      </Typography>
    </Layout>
  );
};

export default Widget;
