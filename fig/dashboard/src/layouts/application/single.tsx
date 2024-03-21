import { Outlet } from "react-router-dom";
import CssBaseline from "@mui/material/CssBaseline";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import { ThemeProvider } from "@mui/material/styles";

import { theme as defaultTheme } from "..";
import Copyright from "../Copyright";

export function Component() {
  return (
    <ThemeProvider theme={defaultTheme}>
      <Container component="main" maxWidth="sm">
        <CssBaseline />
        <Box
          sx={{
            marginTop: 8,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
          }}
        >
          <Outlet />
        </Box>
        <Copyright />
      </Container>
    </ThemeProvider>
  );
}
