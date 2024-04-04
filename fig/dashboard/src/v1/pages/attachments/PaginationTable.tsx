import Typography from "@mui/material/Typography";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import { FormattedMessage } from "react-intl";
import Paper from "@mui/material/Paper";

import Moment from "../../components/Moment";
import TablePagination from "../../components/TablePagination";
import { IAttachment, IPagination } from "../../api/camelia";
import ShowButton from "./:id/ShowButton";
import EditButton from "./:id/EditButton";

interface IProps {
  items: IAttachment[];
  pagination: IPagination;
  handleRefresh: (page: number, size: number) => void;
}

const Widget = ({ items, pagination, handleRefresh }: IProps) => {
  return (
    <>
      <Typography component="h2" variant="h6" color="primary" gutterBottom>
        <FormattedMessage id="attachments.index.title" />
      </Typography>
      <TableContainer component={Paper}>
        <Table size="small">
          <TableHead>
            <TableRow>
              <TableCell align="center">
                <FormattedMessage id="form.fields.id.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.title.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.content-type.label" />
              </TableCell>
              <TableCell>
                <FormattedMessage id="form.fields.updated-at.label" />
              </TableCell>
              <TableCell />
            </TableRow>
          </TableHead>
          <TableBody>
            {items.map((it, id) => (
              <TableRow hover key={id}>
                <TableCell align="center">{it.id}</TableCell>
                <TableCell>{it.title}</TableCell>
                <TableCell>{it.contentType}</TableCell>
                <TableCell>
                  <Moment date={it.updatedAt} />
                </TableCell>
                <TableCell>
                  <ShowButton item={it} small />
                  <EditButton
                    item={it}
                    handleRefresh={() => handleRefresh(1, 20)}
                    small
                  />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
          <TablePagination
            size={pagination.size}
            total={pagination.total}
            page={pagination.page}
            handleChange={handleRefresh}
          />
        </Table>
      </TableContainer>
    </>
  );
};

export default Widget;
