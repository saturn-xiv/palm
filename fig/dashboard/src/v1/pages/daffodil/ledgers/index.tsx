import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";

import NavBar from "./NavBar";
import Show from "./ItemCard";
import { index_ledger, ILedger } from "../../../api/daffodil";

export function Component() {
  const [items, setItems] = useState<ILedger[]>([]);
  useEffect(() => {
    index_ledger().then((res) => setItems(res));
  }, []);
  return (
    <>
      <Grid item xs={12}>
        <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
          <NavBar />
        </Paper>
      </Grid>
      {items.map((it, id) => (
        <Grid key={id} item xs={12} md={4}>
          <Show item={it} />
        </Grid>
      ))}
    </>
  );
}
