import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import AddOutlinedIcon from "@mui/icons-material/AddOutlined";
import LabelImportantOutlinedIcon from "@mui/icons-material/LabelImportantOutlined";
import { useNavigate } from "react-router-dom";
import { FormattedMessage } from "react-intl";

const Widget = () => {
  const navigate = useNavigate();
  return (
    <Stack spacing={2} direction="row">
      <Button
        onClick={() => navigate("/dashboard/daffodil/ledgers/new")}
        variant="contained"
        startIcon={<AddOutlinedIcon />}
      >
        <FormattedMessage id="buttons.new" />
      </Button>
      <Button
        variant="contained"
        startIcon={<LabelImportantOutlinedIcon />}
        color="inherit"
      >
        <FormattedMessage id="buttons.import" />
      </Button>
    </Stack>
  );
};

export default Widget;
