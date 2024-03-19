import Link from "@mui/material/Link";
import Typography from "@mui/material/Typography";
import { FormattedMessage } from "react-intl";

const Widget = () => {
  // CHANGE ME
  const title = "Change me";
  const home = "change-me";
  return (
    <Typography
      variant="body2"
      color="text.secondary"
      align="center"
      sx={{ mt: 8, mb: 4 }}
    >
      <FormattedMessage id="layouts.copyright" />
      &nbsp;
      <Link color="inherit" href={home}>
        {title}
      </Link>
      &nbsp; 2024~{new Date().getFullYear()}.
    </Typography>
  );
};

export default Widget;
