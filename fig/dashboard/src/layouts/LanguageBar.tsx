import { useEffect } from "react";
import Link from "@mui/material/Link";
import { FormattedMessage } from "react-intl";

import { useAppSelector, useAppDispatch } from "../hooks";
import {
  refresh as refreshLayout,
  selectSiteInfo,
} from "../reducers/site-info";
import { fetch_layout, current_user } from "../api/camelia";
import { get as get_locale, set as set_locale } from "../locales";
import {
  get as get_token,
  refresh as refreshUser,
  selectIsSignedIn,
  signOut,
} from "../reducers/current-user";

interface IProps {
  languages: string[];
}

const Widget = ({ languages }: IProps) => {
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
      <FormattedMessage id="layouts.language-bar.other-languages" />
      {languages.map((l, i) => (
        <span key={i} style={{ paddingLeft: "4px" }}>
          <Link
            onClick={(e) => {
              e.preventDefault();
              set_locale(l, true);
            }}
            underline="hover"
          >
            <FormattedMessage id={`languages.${l}`} />
          </Link>
        </span>
      ))}
    </>
  );
};

export default Widget;
