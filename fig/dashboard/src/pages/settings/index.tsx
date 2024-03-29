import { useState } from "react";
import Tab from "@mui/material/Tab";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import Box from "@mui/material/Box";
import { FormattedMessage } from "react-intl";

import {
  siteInfo as selectSiteInfo,
  refresh as refreshLayout,
} from "../../reducers/site-info";
import { useAppSelector, useAppDispatch } from "../../hooks";
import { fetch_layout } from "../../api/camelia";
import { get as get_locale } from "../../locales";
import StatusPostgreSql from "./StatusPostgreSql";
import StatusRedis from "./StatusRedis";
import SeoBaidu from "./SeoBaidu";
import SeoGoogle from "./SeoGoogle";
import SeoIndexNow from "./SeoIndexNow";
import SeoRss from "./SeoRss";
import SeoSitemap from "./SeoSitemap";
import SeoPing from "./SeoPing";
import SeoReCaptcha from "./SeoReCaptcha";
import SiteAuthors from "./SiteAuthors";
import SiteBaseInfo from "./SiteBaseInfo";
import SiteGabCode from "./SiteGabCode";
import SiteIcpCode from "./SiteIcpCode";
import SiteKeywords from "./SiteKeywords";
import SiteFavicon from "./SiteFavicon";

const STATUS = "status";
const INFO = "info";
const SEO = "seo";

export function Component() {
  const dispatch = useAppDispatch();
  const site_info = useAppSelector(selectSiteInfo);
  const [open, setOpen] = useState(STATUS);
  const handleRefresh = () => {
    fetch_layout(get_locale()).then((res) => {
      dispatch(refreshLayout(res));
    });
  };

  return (
    <Grid item xs={12}>
      <Paper sx={{ p: 2, display: "flex", flexDirection: "column" }}>
        <TabContext value={open}>
          <Box sx={{ borderBottom: 1, borderColor: "divider" }}>
            <TabList
              onChange={(_e, v) => {
                setOpen(v);
              }}
              aria-label="lab"
            >
              <Tab
                label={<FormattedMessage id="settings.menus.status" />}
                value={STATUS}
              />
              <Tab
                label={<FormattedMessage id="settings.menus.info" />}
                value={INFO}
              />
              <Tab
                label={<FormattedMessage id="settings.menus.seo" />}
                value={SEO}
              />
            </TabList>
          </Box>
          <TabPanel value={STATUS}>
            <Grid container spacing={2}>
              <Grid item xs={12} md={6}>
                <StatusPostgreSql />
              </Grid>
              <Grid item xs={12} md={6}>
                <StatusRedis />
              </Grid>
            </Grid>
          </TabPanel>
          <TabPanel value={INFO}>
            <Grid container spacing={2}>
              <Grid item xs={12} md={6}>
                <SiteBaseInfo
                  subhead={site_info.subhead}
                  title={site_info.title}
                  copyright={site_info.copyright}
                  description={site_info.description}
                  handleRefresh={handleRefresh}
                />
              </Grid>
              <Grid item xs={12} md={6}>
                <SiteAuthors
                  authors={site_info.authors}
                  handleRefresh={handleRefresh}
                />
              </Grid>
              <Grid item xs={12} md={6}>
                <SiteKeywords
                  keywords={site_info.keywords}
                  handleRefresh={handleRefresh}
                />
              </Grid>
              <Grid item xs={12} md={6}>
                <SiteFavicon
                  url={site_info.favicon}
                  handleRefresh={handleRefresh}
                />
              </Grid>
              <Grid item xs={12} md={6}>
                <SiteIcpCode
                  code={site_info.icpCode}
                  handleRefresh={handleRefresh}
                />
              </Grid>
              <Grid item xs={12} md={6}>
                <SiteGabCode
                  item={site_info.gabCode}
                  handleRefresh={handleRefresh}
                />
              </Grid>
            </Grid>
          </TabPanel>
          <TabPanel value={SEO}>
            <Grid container spacing={2}>
              <Grid item xs={12} md={6}>
                <SeoGoogle />
              </Grid>
              <Grid item xs={12} md={6}>
                <SeoIndexNow />
              </Grid>
              <Grid item xs={12} md={6}>
                <SeoBaidu />
              </Grid>
              <Grid item xs={12} md={6}>
                <SeoReCaptcha />
              </Grid>
              <Grid item xs={12} md={6}>
                <SeoPing />
              </Grid>
              <Grid item xs={12} md={6}>
                <SeoSitemap languages={site_info.languages} />
              </Grid>
              <Grid item xs={12} md={6}>
                <SeoRss languages={site_info.languages} />
              </Grid>
            </Grid>
          </TabPanel>
        </TabContext>
      </Paper>
    </Grid>
  );
}
