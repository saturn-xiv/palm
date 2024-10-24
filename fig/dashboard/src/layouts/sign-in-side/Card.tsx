import { FormEventHandler, ReactNode } from "react";
import Box from "@mui/material/Box";
import MuiCard from "@mui/material/Card";
import Divider from "@mui/material/Divider";
import Typography from "@mui/material/Typography";
import { styled } from "@mui/material/styles";
import { FormattedMessage } from "react-intl";

import ByGoogle from "./ByGoogle";
import ByFacebook from "./ByFacebook";

interface IProps {
  title: string;
  handleSubmit: FormEventHandler<HTMLFormElement>;
  children: ReactNode;
}

const Card = styled(MuiCard)(({ theme }) => ({
  display: "flex",
  flexDirection: "column",
  alignSelf: "center",
  width: "100%",
  padding: theme.spacing(4),
  gap: theme.spacing(2),
  boxShadow:
    "hsla(220, 30%, 5%, 0.05) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.05) 0px 15px 35px -5px",
  [theme.breakpoints.up("sm")]: {
    width: "450px",
  },
  ...theme.applyStyles("dark", {
    boxShadow:
      "hsla(220, 30%, 5%, 0.5) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.08) 0px 15px 35px -5px",
  }),
}));

const Widget = ({ title, children, handleSubmit }: IProps) => {
  return (
    <Card variant="outlined">
      <Typography
        component="h1"
        variant="h4"
        sx={{ width: "100%", fontSize: "clamp(2rem, 10vw, 2.15rem)" }}
      >
        {title}
      </Typography>
      <Box
        component="form"
        onSubmit={handleSubmit}
        noValidate
        sx={{ display: "flex", flexDirection: "column", width: "100%", gap: 2 }}
      >
        {children}
      </Box>
      <Divider>
        <FormattedMessage id="buttons.or" />
      </Divider>
      <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
        <ByGoogle />
        <ByFacebook />
      </Box>
    </Card>
  );
};

export default Widget;
