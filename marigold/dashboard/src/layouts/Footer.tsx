import { useEffect } from "react";

import { useAppSelector } from "../hooks";
import { selectLayout } from "../reducers/layout";
import { refresh } from "../api/portal/user";
import { get as get_locale } from "../locales";

const Widget = () => {
  const site = useAppSelector(selectLayout);

  useEffect(() => {
    const call = async () => {
      if (!site?.project.version) {
        // TODO
        refresh(get_locale());
        console.log("fetch site layout");
      }
    };
    call();
  }, [site]);
  return <>{site?.copyright || ""}</>;
};

export default Widget;
