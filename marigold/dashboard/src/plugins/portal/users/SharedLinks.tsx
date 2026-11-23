import { type SyntheticEvent } from "react";
import Box from "@mui/material/Box";
import Link from "@mui/material/Link";
import Typography from "@mui/material/Typography";
import { FormattedMessage } from "react-intl";
import { useNavigate } from "react-router";

interface IItem {
  href: string;
  label: string;
}

const Widget = () => {
  const navigate = useNavigate();
  const items: IItem[] = [
    {
      href: "/anonymous/users/sign-in",
      label: "portal.pages.anonymous.users.sign-in.title",
    },
    {
      href: "/anonymous/users/sign-up",
      label: "portal.pages.anonymous.users.sign-up.title",
    },
    {
      href: "/anonymous/users/by-email/confirm",
      label: "portal.pages.anonymous.users.by-email.confirm.title",
    },
    {
      href: "/anonymous/users/by-email/unlock",
      label: "portal.pages.anonymous.users.by-email.unlock.title",
    },
    {
      href: "/anonymous/users/by-email/forgot-password",
      label: "portal.pages.anonymous.users.by-email.forgot-password.title",
    },
  ];
  return (
    <Box
      sx={{ display: "flex", flexDirection: "column", width: "100%", gap: 2 }}
    >
      {items.map((x, i) => (
        <Typography sx={{ textAlign: "center" }} key={i}>
          <Link
            onClick={(event: SyntheticEvent) => {
              event.preventDefault();
              navigate(x.href);
            }}
            href="#"
            variant="body2"
            sx={{ alignSelf: "center" }}
          >
            <FormattedMessage id={x.label} />
          </Link>
        </Typography>
      ))}
    </Box>
  );
};

export default Widget;
