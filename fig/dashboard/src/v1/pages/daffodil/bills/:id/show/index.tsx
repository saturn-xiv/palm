import { useState, useEffect } from "react";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import { useParams } from "react-router-dom";

import {
  IBill,
  BILL_RESOURCE_TYPE,
  show_bill,
} from "../../../../../api/daffodil";
import NavBar from "./NavBar";
import ShowCard from "./Card";
import AttachmentTable from "../../../../attachments/RawTable";
import Upload from "../../../../attachments/Upload";

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
          <AttachmentTable
            handleRefresh={() => {
              show_bill(item.id).then((res) => {
                setItem(res);
              });
            }}
            items={item.attachments}
          />
          <br />
          <Upload
            accept={{
              "image/png": [".png"],
              "image/jpeg": [".jpg", ".jpeg"],
              "application/pdf": [".pdf"],
            }}
            resource={{ type: BILL_RESOURCE_TYPE, id: item.id }}
            handleRefresh={() => {
              show_bill(item.id).then((res) => {
                setItem(res);
              });
            }}
          />
        </Paper>
      </Grid>
    </>
  ) : (
    <></>
  );
}
