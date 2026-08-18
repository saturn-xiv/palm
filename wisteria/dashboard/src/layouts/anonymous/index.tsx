import { Outlet } from "react-router";
import Stack from "@mui/material/Stack";
import MuiCard from "@mui/material/Card";

import SharedLinks from "./SharedLinks";

const Widget = () => {
  return (
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
          <SharedLinks />
          <MuiCard variant="outlined">
            <Outlet />
          </MuiCard>
        </Stack>
      </Stack>
    </Stack>
  );
};

export default Widget;
