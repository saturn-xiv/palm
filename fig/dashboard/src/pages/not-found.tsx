import CssBaseline from "@mui/material/CssBaseline";
import { ThemeProvider } from "@mui/material/styles";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Container from "@mui/material/Container";
import { FormattedMessage } from "react-intl";

import { theme as defaultTheme } from "../layouts";
import Copyright from "../layouts/Copyright";
import not_found_img from "../assets/404.svg";

export function Component() {
  return (
    <ThemeProvider theme={defaultTheme}>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          minHeight: "100vh",
        }}
      >
        <CssBaseline />
        <Container component="main" sx={{ mt: 8, mb: 2 }} maxWidth="sm">
          <Typography variant="h2" component="h1" gutterBottom>
            <FormattedMessage id="layouts.not-found.title" />
          </Typography>
          <Typography variant="h5" component="h2" gutterBottom>
            <img src={not_found_img} />
          </Typography>
          <Typography variant="body1">
            <FormattedMessage id="layouts.not-found.body" />
          </Typography>
        </Container>
        <Box
          component="footer"
          sx={{
            py: 3,
            px: 2,
            mt: "auto",
            backgroundColor: (theme) =>
              theme.palette.mode === "light"
                ? theme.palette.grey[200]
                : theme.palette.grey[800],
          }}
        >
          <Container maxWidth="sm">
            <Typography variant="body1" />
            <Copyright />
          </Container>
        </Box>
      </Box>
    </ThemeProvider>
  );
}
