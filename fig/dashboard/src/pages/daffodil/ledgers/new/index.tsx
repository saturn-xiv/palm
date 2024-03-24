import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";

import New from "../NewForm";

export function Component() {
  return (
    <Grid item xs={12} md={6}>
      <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
        <New />
      </Paper>
    </Grid>
  );
}
