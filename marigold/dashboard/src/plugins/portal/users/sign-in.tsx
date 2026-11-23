import { useState, type SyntheticEvent } from "react";
import { useIntl } from "react-intl";
import Typography from "@mui/material/Typography";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";

import Title from "../../../layouts/Title";
import SignInByEmail from "./by-email/SignIn";
import SignInBySms from "./by-sms/SignIn";
import SharedLinksByEmail from "./SharedLinks";

const BY_SMS = "by-sms";
const BY_EMAIL = "by-email";

const Widget = () => {
  const intl = useIntl();
  const [tab, setTab] = useState(BY_EMAIL);

  const handleChange = (_event: SyntheticEvent, value: string) => {
    setTab(value);
  };

  return (
    <>
      <Typography
        component="h1"
        variant="h4"
        sx={{ width: "100%", fontSize: "clamp(2rem, 10vw, 2.15rem)" }}
      >
        <Title
          value={intl.formatMessage({
            id: "portal.pages.anonymous.users.sign-in.title",
          })}
        />
      </Typography>
      <TabContext value={tab}>
        <TabList
          onChange={handleChange}
          sx={{ borderBottom: 1, borderColor: "divider" }}
        >
          <Tab
            label={intl.formatMessage({
              id: "portal.pages.anonymous.users.by-email.title",
            })}
            value={BY_EMAIL}
          />
          <Tab
            label={intl.formatMessage({
              id: "portal.pages.anonymous.users.by-sms.title",
            })}
            value={BY_SMS}
          />
        </TabList>
        <TabPanel value={BY_EMAIL} tabIndex={0}>
          <SignInByEmail />
        </TabPanel>
        <TabPanel value={BY_SMS} tabIndex={0}>
          <SignInBySms />
        </TabPanel>
      </TabContext>
      <SharedLinksByEmail />
    </>
  );
};

export default Widget;
