import { useEffect, type SyntheticEvent } from "react";
import Link from "@mui/material/Link";
import Box from "@mui/material/Box";
import { FormattedMessage } from "react-intl";

import { useAppSelector, useAppDispatch } from "../hooks";
import { selectLayout, refresh as layout_refresh } from "../reducers/layout";
import { signIn, get as get_token } from "../reducers/current-user";
import { refresh } from "../api/portal/user";
import { get as get_locale, set as set_locale } from "../locales";

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
    <>
      {site?.copyright || ""}(v{site?.project.version}).
      <Box
        sx={{
          display: "flex",
          flexWrap: "wrap",
          justifyContent: "center",
        }}
      >
        {(site?.languages || []).map((x, i) => (
          <Link
            key={i}
            onClick={(event: SyntheticEvent) => {
              event.preventDefault();
              set_locale(x, true);
            }}
            underline="hover"
          >
            <FormattedMessage id={`languages.${x}`} />
          </Link>
        ))}
      </Box>
    </>
  );
};

export default Widget;
