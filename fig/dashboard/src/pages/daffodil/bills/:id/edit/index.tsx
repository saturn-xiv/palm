import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import { useParams } from "react-router-dom";

import Edit from "./Form";
import { IBill, show_bill } from "../../../../../api/daffodil";

export function Component() {
  const [item, setItem] = useState<IBill | undefined>(undefined);
  const { id } = useParams();
  useEffect(() => {
    if (id) {
      show_bill(parseInt(id, 10)).then((res) => {
        setItem(res);
      });
    }
  }, [id]);
  return item ? (
    <Grid item xs={12} md={6}>
      <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
        <Edit item={item} />
      </Paper>
    </Grid>
  ) : (
    <></>
  );
}
