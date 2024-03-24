import { useEffect } from "react";
import Link from "@mui/material/Link";
import Typography from "@mui/material/Typography";
import { FormattedMessage } from "react-intl";

import { useAppSelector, useAppDispatch } from "../hooks";
import {
  refresh as refreshLayout,
  siteInfo as selectSiteInfo,
} from "../reducers/site-info";
import { fetch_layout, current_user } from "../api/camelia";
import { get as get_locale } from "../locales";
import {
  get as get_token,
  refresh as refreshUser,
  isSignedIn as selectIsSignedIn,
  signOut,
} from "../reducers/current-user";
import { home_url } from "../utils";
import LanguageBar from "./LanguageBar";
import MessageBox from "./MessageBox";

const Widget = () => {
  const site_info = useAppSelector(selectSiteInfo);
  const is_signed_in = useAppSelector(selectIsSignedIn);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (site_info.version === "") {
      fetch_layout(get_locale()).then((res) => {
        dispatch(refreshLayout(res));
      });
    }
    const token = get_token();
    if (!is_signed_in && token !== null) {
      current_user()
        .then((res) => {
          dispatch(refreshUser(res));
        })
        .catch(() => {
          dispatch(signOut());
        });
    }
  }, [is_signed_in, dispatch, site_info]);

  return (
    <>
      <Typography
        variant="body2"
        color="text.secondary"
        align="center"
        sx={{ mt: 8, mb: 4 }}
      >
        <FormattedMessage id="layouts.copyright" />
        &nbsp;
        <Link color="inherit" href={home_url()}>
          {site_info.title}
        </Link>
        &nbsp; 2024~{new Date().getFullYear()}({site_info.version}).
        <br />
        <LanguageBar languages={site_info.languages} />
      </Typography>
      <MessageBox />
    </>
  );
};

export default Widget;
