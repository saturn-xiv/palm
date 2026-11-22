import { Outlet } from "react-router";
import Stack from "@mui/material/Stack";
import Divider from "@mui/material/Divider";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";

import Layout from "../";
import Description from "./Description";
import Card from "./Card";
import { GoogleIcon, FacebookIcon, WechatIcon, SitemarkIcon } from "./Icons";

// https://github.com/mui/material-ui/tree/v9.5.0/docs/data/material/getting-started/templates/sign-in-side
const Widget = () => {
  return (
    <Layout>
      <Stack
        direction="column"
        component="main"
        sx={[
          {
            justifyContent: "center",
            height: "calc((1 - var(--template-frame-height, 0)) * 100%)",
            marginTop: "max(40px - var(--template-frame-height, 0px), 0px)",
            minHeight: "100%",
          },
          (theme) => ({
            "&::before": {
              content: '""',
              display: "block",
              position: "absolute",
              zIndex: -1,
              inset: 0,
              backgroundImage:
                "radial-gradient(ellipse at 50% 50%, hsl(210, 100%, 97%), hsl(0, 0%, 100%))",
              backgroundRepeat: "no-repeat",
              ...theme.applyStyles("dark", {
                backgroundImage:
                  "radial-gradient(at 50% 50%, hsla(210, 100%, 16%, 0.5), hsl(220, 30%, 5%))",
              }),
            },
          }),
        ]}
      >
        <Stack
          direction={{ xs: "column-reverse", md: "row" }}
          sx={{
            justifyContent: "center",
            gap: { xs: 6, sm: 12 },
            p: 2,
            mx: "auto",
          }}
        >
          <Stack
            direction={{ xs: "column-reverse", md: "row" }}
            sx={{
              justifyContent: "center",
              gap: { xs: 6, sm: 12 },
              p: { xs: 2, sm: 4 },
              m: "auto",
            }}
          >
            <Description />
            <Card variant="outlined">
              <Box sx={{ display: { xs: "flex", md: "none" } }}>
                <SitemarkIcon />
              </Box>
              <Outlet />
              <Divider>or</Divider>
              <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
                <Button
                  fullWidth
                  variant="outlined"
                  onClick={() => alert("Sign in with Google")}
                  startIcon={<GoogleIcon />}
                >
                  Sign in with Google
                </Button>
                <Button
                  fullWidth
                  variant="outlined"
                  onClick={() => alert("Sign in with Facebook")}
                  startIcon={<FacebookIcon />}
                >
                  Sign in with Facebook
                </Button>
                <Button
                  fullWidth
                  variant="outlined"
                  onClick={() => alert("Sign in with Wechat")}
                  startIcon={<WechatIcon />}
                >
                  Sign in with Wechat
                </Button>
              </Box>
            </Card>
          </Stack>
        </Stack>
      </Stack>
    </Layout>
  );
};

export default Widget;
