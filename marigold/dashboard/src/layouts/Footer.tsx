import { useEffect } from "react";

import { useAppSelector } from "../hooks";
import { selectLayout } from "../reducers/layout";
import { refresh } from "../api/portal/user";

const Widget = () => {
  const site = useAppSelector(selectLayout);

  useEffect(() => {
    const call = async () => {
      if (!site?.version) {
        refresh;
        console.log("fetch site layout");
      }
    };
    call();
  }, [site]);
  return <>{site?.copyright || ""}</>;
};

export default Widget;
