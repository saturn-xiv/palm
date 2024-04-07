import { useEffect } from "react";

import { useAppDispatch } from "../../../../hooks";
import { set_pathname } from "../../../../reducers/side-bar";
import { SETTINGS_SITE_SEO_PATH } from "../../../../Router";

export const Component = () => {
  const dispatch = useAppDispatch();
  useEffect(() => {
    dispatch(set_pathname(SETTINGS_SITE_SEO_PATH));
  }, [dispatch]);
  return <>atta</>;
};
