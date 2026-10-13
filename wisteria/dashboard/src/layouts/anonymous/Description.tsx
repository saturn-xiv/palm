import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import AutoFixHighRoundedIcon from "@mui/icons-material/AutoFixHighRounded";
import ConstructionRoundedIcon from "@mui/icons-material/ConstructionRounded";
import SettingsSuggestRoundedIcon from "@mui/icons-material/SettingsSuggestRounded";
import ThumbUpAltRoundedIcon from "@mui/icons-material/ThumbUpAltRounded";
import { useIntl } from "react-intl";

import { SitemarkIcon } from "./Icons";
import Footer from "../Footer";

const Widget = () => {
  const intl = useIntl();
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
        <SitemarkIcon />
      </Box>
      {[
        {
          icon: <SettingsSuggestRoundedIcon sx={{ color: "text.secondary" }} />,
          title: intl.formatMessage({
            id: "portal.layouts.anonymous.description.settings-suggest.title",
          }),
          description:
            "portal.layouts.anonymous.description.settings-suggest.content",
        },
        {
          icon: <ConstructionRoundedIcon sx={{ color: "text.secondary" }} />,
          title: "portal.layouts.anonymous.description.construction.title",
          description:
            "portal.layouts.anonymous.description.construction.content",
        },
        {
          icon: <ThumbUpAltRoundedIcon sx={{ color: "text.secondary" }} />,
          title: "portal.layouts.anonymous.description.thumb-up-alt.title",
          description:
            "portal.layouts.anonymous.description.thumb-up-alt.content",
        },
        {
          icon: <AutoFixHighRoundedIcon sx={{ color: "text.secondary" }} />,
          title: "portal.layouts.anonymous.description.auto-fix-high.title",
          description:
            "portal.layouts.anonymous.description.auto-fix-high.content",
        },
      ].map((item, index) => (
        <Stack key={index} direction="row" sx={{ gap: 2 }}>
          {item.icon}
          <div>
            <Typography gutterBottom sx={{ fontWeight: "medium" }}>
              {item.title}
            </Typography>
            <Typography variant="body2" sx={{ color: "text.secondary" }}>
              {item.description}
            </Typography>
          </div>
        </Stack>
      ))}
      <Box sx={{ display: { xs: "none", md: "flex" } }}>
        <Footer />
      </Box>
    </Stack>
  );
};

export default Widget;
