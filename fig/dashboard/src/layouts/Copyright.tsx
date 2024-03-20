import { useEffect } from "react";
import Link from "@mui/material/Link";
import Typography from "@mui/material/Typography";
import { FormattedMessage } from "react-intl";

import { useAppSelector, useAppDispatch } from "../hooks";
import { refresh, selectSiteInfo } from "../reducers/site-info";
import { fetch_layout, ILayoutResponse } from "../api/camelia";
import { get as get_locale } from "../locales";

const Widget = () => {
  const site_info = useAppSelector(selectSiteInfo);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (site_info.version === "") {
      fetch_layout(get_locale()).then((res: ILayoutResponse) => {
        dispatch(refresh(res));
      });
    }
  });

  const home = "change-me";
  return (
    <Typography
      variant="body2"
      color="text.secondary"
      align="center"
      sx={{ mt: 8, mb: 4 }}
    >
      <FormattedMessage id="layouts.copyright" />
      &nbsp;
      <Link color="inherit" href={home}>
        {site_info.title}
      </Link>
      &nbsp; 2024~{new Date().getFullYear()}({site_info.version}).
    </Typography>
  );
};

export default Widget;
