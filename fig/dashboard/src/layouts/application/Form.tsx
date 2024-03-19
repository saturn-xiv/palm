import { FormEventHandler, ReactNode } from "react";
import { FormattedMessage } from "react-intl";
import Avatar from "@mui/material/Avatar";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

import SharedLinks from "./SharedLinks";

interface IProps {
  handleSubmit: FormEventHandler<HTMLFormElement>;
  title: string;
  icon: ReactNode;
  children: ReactNode;
}

const Widget = ({ icon, title, handleSubmit, children }: IProps) => {
  return (
    <>
      <Avatar sx={{ m: 1, bgcolor: "secondary.main" }}>{icon}</Avatar>
      <Typography component="h1" variant="h5">
        <FormattedMessage id={title} />
      </Typography>
      <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
        {children}
        <Button
          type="submit"
          fullWidth
          variant="contained"
          sx={{ mt: 3, mb: 2 }}
        >
          <FormattedMessage id="buttons.submit" />
        </Button>
        <SharedLinks />
      </Box>
    </>
  );
};

export default Widget;
