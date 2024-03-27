import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import EditOutlinedIcon from "@mui/icons-material/EditOutlined";
import ArrowBackOutlinedIcon from "@mui/icons-material/ArrowBackOutlined";
import { FormattedMessage } from "react-intl";
import { useNavigate } from "react-router-dom";

import { IBill } from "../../../../../api/daffodil";
import DestroyDialog from "../DestroyDialog";

interface IProps {
  item: IBill;
}

const Widget = ({ item }: IProps) => {
  const navigate = useNavigate();
  return (
    <Stack spacing={2} direction="row">
      <Button
        onClick={() => navigate(`/dashboard/daffodil/bills/${item.id}/edit`)}
        variant="contained"
        startIcon={<EditOutlinedIcon />}
      >
        <FormattedMessage id="buttons.edit" />
      </Button>
      <DestroyDialog item={item} />
      <Button
        variant="contained"
        startIcon={<ArrowBackOutlinedIcon />}
        color="inherit"
        onClick={() => navigate(`/dashboard/daffodil/ledgers/${item.ledger}`)}
      >
        <FormattedMessage id="buttons.go-back" />
      </Button>
    </Stack>
  );
};

export default Widget;
