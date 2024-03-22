import { useState } from "react";
import TableFooter from "@mui/material/TableFooter";
import TableRow from "@mui/material/TableRow";
import TablePagination from "@mui/material/TablePagination";

export interface IProps {
  page: number;
  size: number;
  total: number;
  handleChange: (page: number, size: number) => void;
}

const Widget = (props: IProps) => {
  const [size, setSize] = useState<number>(props.size);
  const [page, setPage] = useState<number>(props.page - 1);
  return (
    <TableFooter>
      <TableRow>
        <TablePagination
          rowsPerPageOptions={[5, 10, 20, 30, 50]}
          colSpan={3}
          count={props.total}
          rowsPerPage={size}
          page={page}
          slotProps={{
            select: {
              inputProps: {
                "aria-label": "rows per page",
              },
              native: true,
            },
          }}
          onPageChange={(_e, p) => {
            setPage(p);
            props.handleChange(p + 1, size);
          }}
          onRowsPerPageChange={(e) => {
            const s = parseInt(e.target.value, 10);
            setSize(s);
            setPage(0);
            props.handleChange(0 + 1, s);
          }}
        />
      </TableRow>
    </TableFooter>
  );
};

export default Widget;
