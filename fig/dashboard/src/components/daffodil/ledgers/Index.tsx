import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";

import Show from "./Show";
import { index_ledger, ILedger } from "../../../api/daffodil";

const Widget = () => {
  const [items, setItems] = useState<ILedger[]>([]);
  useEffect(() => {
    index_ledger().then((res) => setItems(res));
  }, []);
  return (
    <>
      {items.map((it, id) => (
        <Grid key={id} item xs={12} md={3}>
          <Show item={it} />
        </Grid>
      ))}
    </>
  );
};

export default Widget;
