import { useEffect, useState } from "react";
import Typography from "@mui/material/Typography";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";
import { FormattedMessage } from "react-intl";
import { useNavigate } from "react-router-dom";

import { IBill, ILedger, bills_by_ledger } from "../../../api/daffodil";
import AmountShow from "./:id/show/Amount";
import Moment from "../../../components/Moment";

interface IProps {
  ledger: ILedger;
}

const Widget = ({ ledger }: IProps) => {
  const navigate = useNavigate();
  const [items, setItems] = useState<IBill[]>([]);
  useEffect(() => {
    bills_by_ledger(ledger.id).then((res) => setItems(res));
  }, [ledger]);
  return (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="daffodil.bills.index.title" />
      </Typography>
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }} size="small">
          <TableHead>
            <TableRow>
              <TableCell align="center">
                <FormattedMessage id="form.fields.id.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.amount.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.summary.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.updated-at.label" />
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {items.map((it, id) => (
              <TableRow
                hover
                onClick={() => {
                  navigate(`/dashboard/daffodil/bills/${it.id}`);
                }}
                key={id}
              >
                <TableCell align="center">{it.id}</TableCell>
                <TableCell>
                  <AmountShow
                    amount={it.amount}
                    currency={it.currency}
                    abbreviation
                  />
                </TableCell>
                <TableCell>{it.summary}</TableCell>
                <TableCell>
                  <Moment date={it.updatedAt} />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </>
  );
};

export default Widget;
