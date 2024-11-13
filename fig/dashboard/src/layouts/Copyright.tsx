import { useEffect } from "react";

import { useAppSelector, useAppDispatch } from "../hooks";
import { refresh } from "../reducers/site";
import { isSignIn, signIn, get as get_token } from "../reducers/current-user";

const Widget = () => {
  const is_sign_in = useAppSelector(isSignIn);
  const token = get_token();
  const site = useAppSelector((state) => state.site.layout);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (!site) {
      // TODO
      dispatch(
        refresh({
          title: "ttt",
          subhead: "sss",
          copyright: "ccc",
          logo: "/my/logo.svg",
        })
      );
    }
    if (!is_sign_in && token !== null) {
      dispatch(signIn({ token }));
    }
  }, [site, dispatch, is_sign_in, token]);
  return <>&copy;{site?.copyright}</>;
};

export default Widget;
