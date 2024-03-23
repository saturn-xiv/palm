import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";

import NavBar from "../../../components/daffodil/ledgers/NavBar";
import Index from "../../../components/daffodil/ledgers/Index";

export function Component() {
  return (
    <>
      <Grid item xs={12}>
        <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
          <NavBar />
        </Paper>
      </Grid>

      <Index />
    </>
  );
}
