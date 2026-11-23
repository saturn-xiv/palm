import { useEffect, type SyntheticEvent } from "react";
import Link from "@mui/material/Link";
import Box from "@mui/material/Box";
import { FormattedMessage } from "react-intl";

import { useAppSelector, useAppDispatch } from "../hooks";
import { selectLayout, refresh as layout_refresh } from "../reducers/layout";
import { signIn, get as get_token } from "../reducers/current-user";
import { refresh } from "../api/portal/user";
import { get as get_locale, set as set_locale } from "../locales";
import { type IProject } from "../api/portal/site";

const SourceLicensedBy = () => (
  <>
    The source code is licensed&nbsp;
    <Link
      href="https://opensource.org/license/mit"
      target="_blank"
      rel="noopener noreferrer"
    >
      MIT
    </Link>
    .
  </>
);

const WebsiteContentLicensedBy = () => (
  <>
    The website content is licensed&nbsp;
    <Link
      href="https://creativecommons.org/licenses/by-nc-sa/4.0//"
      target="_blank"
      rel="noopener noreferrer"
    >
      CC BY NC SA 4.0
    </Link>
    .
  </>
);

const PoweredBy = ({ project }: { project: IProject }) => (
  <>
    Powered by&nbsp;
    <Link href={project.url || "#"} target="_blank" rel="noopener noreferrer">
      {project.name || ""}
    </Link>
    .
  </>
);

const Copyright = ({
  copyright,
  version,
}: {
  copyright: string;
  version: string;
}) => (
  <>
    {copyright}(v{version}).
  </>
);

const Language = ({ lang }: { lang: string }) => (
  <Link
    onClick={(event: SyntheticEvent) => {
      event.preventDefault();
      set_locale(lang, true);
    }}
    sx={{ px: 1 }}
    underline="hover"
  >
    <FormattedMessage id={`languages.${lang}`} />
  </Link>
);

const Widget = () => {
  const site = useAppSelector(selectLayout);
  const dispatch = useAppDispatch();

  useEffect(() => {
    const call = async () => {
      if (!site?.buildTime) {
        const res = await refresh(get_locale());
        dispatch(layout_refresh(res.page));
        const token = get_token();
        if (res.user && token) {
          dispatch(signIn({ token, user: res.user }));
        }
      }
    };
    call();
  }, [site, dispatch]);
  return (
    <Box
      sx={{
        display: "flex",
        flexWrap: "wrap",
        justifyContent: "center",
      }}
    >
      <Copyright
        copyright={site?.copyright || ""}
        version={site?.project.version || ""}
      />
      <PoweredBy
        project={site?.project || { name: "", url: "#", version: "" }}
      />
      <SourceLicensedBy />
      <WebsiteContentLicensedBy />
      {(site?.languages || []).map((x, i) => (
        <Language lang={x} key={i} />
      ))}
    </Box>
  );
};

export default Widget;
