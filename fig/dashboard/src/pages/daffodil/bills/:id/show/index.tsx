import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import { useParams } from "react-router-dom";

import { IBill, show_bill } from "../../../../../api/daffodil";
import NavBar from "./NavBar";
import ShowCard from "./Card";
import AttachmentTable from "../../../../attachments/RawTable";

export function Component() {
  const [item, setItem] = useState<IBill | undefined>();
  const { id } = useParams();
  useEffect(() => {
    if (id) {
      show_bill(parseInt(id, 10)).then((res) => {
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
      <Grid item xs={12} md={3}>
        <ShowCard item={item} />
      </Grid>
      <Grid item xs={12} md={9}>
        <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
          <AttachmentTable items={item.attachments} />
        </Paper>
      </Grid>
    </>
  ) : (
    <></>
  );
}
