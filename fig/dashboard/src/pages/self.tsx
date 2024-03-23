import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";

import UserLogs from "../components/users/Logs";
import UserUpdateProfile from "../components/users/UpdateProfile";
import UserChangePassword from "../components/users/ChangePassword";
import IndexAttachment from "../components/attachments/Index";

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
      <Grid item xs={12}>
        <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
          <IndexAttachment />
        </Paper>
      </Grid>
    </>
  );
}
