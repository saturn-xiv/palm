import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";

import UserLogs from "./Logs";
import UserUpdateProfile from "./UpdateProfile";
import UserChangePassword from "./ChangePassword";

export function Component() {
  return (
    <>
      <Grid item xs={12}>
        <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
          <UserLogs />
        </Paper>
      </Grid>
      <Grid item xs={12} md={6}>
        <Paper
          sx={{
            p: 2,
            display: "flex",
            flexDirection: "column",
            height: 560,
          }}
        >
          <UserUpdateProfile />
        </Paper>
      </Grid>
      <Grid item xs={12} md={6}>
        <Paper
          sx={{
            p: 2,
            display: "flex",
            flexDirection: "column",
            height: 480,
          }}
        >
          <UserChangePassword />
        </Paper>
      </Grid>
    </>
  );
}
