import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import { useParams } from "react-router-dom";

import { ILedger, show_ledger } from "../../../../../api/daffodil";
import NavBar from "./NavBar";
import Bills from "../../../bills/Table";

export function Component() {
  const [item, setItem] = useState<ILedger | undefined>();
  const { id } = useParams();
  useEffect(() => {
    if (id) {
      show_ledger(parseInt(id, 10)).then((res) => {
        setItem(res);
      });
    }
  }, [id]);
  return item ? (
    <>
      <Grid item xs={12}>
        <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
          <NavBar item={item} />
        </Paper>
      </Grid>
      <Grid item xs={12}>
        <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
          <Bills ledger={item} />
        </Paper>
      </Grid>
    </>
  ) : (
    <></>
  );
}
