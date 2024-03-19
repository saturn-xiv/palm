import { useNavigate } from "react-router-dom";
import { FormattedMessage } from "react-intl";
import Link from "@mui/material/Link";
import Grid from "@mui/material/Grid";

const links = [
  {
    to: "/anonymous/users/sign-in",
    title: "users.sign-in.title",
  },
  {
    to: "/anonymous/users/sign-up",
    title: "users.sign-up.title",
  },
  {
    to: "/anonymous/users/confirm",
    title: "users.confirm.by-email.title",
  },
  {
    to: "/anonymous/users/unlock",
    title: "users.unlock.by-email.title",
  },
  {
    to: "/anonymous/users/forgot-password",
    title: "users.forgot-password.title",
  },
  {
    to: "/anonymous/leave-words/new",
    title: "leave-words.new.title",
  },
];

function Widget() {
  const navigate = useNavigate();
  return (
    <Grid container>
      {links.map((x) => (
        <Grid key={x.to} item md={6} xs={12}>
          <Link onClick={() => navigate(x.to)} variant="body2">
            <FormattedMessage id={x.title} />
          </Link>
        </Grid>
      ))}
    </Grid>
  );
}

export default Widget;
