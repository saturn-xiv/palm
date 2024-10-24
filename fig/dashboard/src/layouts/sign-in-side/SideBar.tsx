import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Link from "@mui/material/Link";
import Typography from "@mui/material/Typography";
import LoginRoundedIcon from "@mui/icons-material/LoginRounded";
import PasswordRoundedIcon from "@mui/icons-material/PasswordRounded";
import PersonAddRoundedIcon from "@mui/icons-material/PersonAddRounded";
import VerifiedUserRoundedIcon from "@mui/icons-material/VerifiedUserRounded";
import LockOpenRoundedIcon from "@mui/icons-material/LockOpenRounded";
import AddCommentRoundedIcon from "@mui/icons-material/AddCommentRounded";
import { useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { useAppSelector } from "../../hooks";
import Copyright from "../Copyright";

const Widget = () => {
  const intl = useIntl();
  const navigate = useNavigate();
  const layout = useAppSelector((state) => state.site.layout);

  const items = [
    {
      to: "/anonymous/users/sign-in",
      icon: <LoginRoundedIcon sx={{ color: "text.secondary" }} />,
      title: "pages.users.sign-in.title",
    },
    {
      to: "/anonymous/users/sign-up",
      icon: <PersonAddRoundedIcon sx={{ color: "text.secondary" }} />,
      title: "pages.users.sign-up.title",
    },
    {
      to: "/anonymous/users/confirm",
      icon: <VerifiedUserRoundedIcon sx={{ color: "text.secondary" }} />,
      title: "pages.users.confirm.title",
    },
    {
      to: "/anonymous/users/unlock",
      icon: <LockOpenRoundedIcon sx={{ color: "text.secondary" }} />,
      title: "pages.users.unlock.title",
    },
    {
      to: "/anonymous/users/forgot-password",
      icon: <PasswordRoundedIcon sx={{ color: "text.secondary" }} />,
      title: "pages.users.forgot-password.title",
    },
    {
      to: "/anonymous/leave-words/new",
      icon: <AddCommentRoundedIcon sx={{ color: "text.secondary" }} />,
      title: "pages.leave-words.new.title",
    },
  ];
  return (
    <Stack
      sx={{
        flexDirection: "column",
        alignSelf: "center",
        gap: 4,
        maxWidth: 450,
      }}
    >
      <Box sx={{ display: { xs: "none", md: "flex" } }}>
        <img
          style={{
            height: 24,
            width: 24,
          }}
          src={layout?.logo}
        />
        &nbsp; {layout?.subhead}
      </Box>
      {items.map((item) => (
        <Stack key={item.to} direction="row" sx={{ gap: 2 }}>
          {item.icon}
          <Typography gutterBottom sx={{ fontWeight: "medium" }}>
            <Link
              onClick={(e) => {
                e.preventDefault();
                navigate(item.to);
              }}
            >
              {intl.formatMessage({ id: item.title })}
            </Link>
          </Typography>
        </Stack>
      ))}
      <Stack direction="row" sx={{ gap: 2 }}>
        <Copyright />
      </Stack>
    </Stack>
  );
};
export default Widget;
