import { useEffect } from "react";

import { useAppSelector, useAppDispatch } from "../hooks";
import { refresh as refresh_site, siteInfo } from "../reducers/site";
import { isSignIn, signIn, get as get_token } from "../reducers/current-user";
import { refresh } from "../api/daffodil";
import { IError } from "../api";

const Widget = () => {
  const is_sign_in = useAppSelector(isSignIn);
  const token = get_token();
  const site = useAppSelector(siteInfo);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (site === undefined) {
      refresh()
        .then((res) => {
          dispatch(refresh_site(res.siteInfo));
          if (token && res.currentUser) {
            dispatch(signIn({ token, profile: res.currentUser }));
          }
        })
        .catch((reason: IError[]) => {
          console.error(reason);
        });
    }
  }, [site, dispatch, is_sign_in, token]);
  return <>&copy;{site?.copyright}</>;
};

export default Widget;
