import { useEffect } from "react";

import { useAppSelector, useAppDispatch } from "../hooks";
import { refresh } from "../reducers/site";

const Widget = () => {
  const site_layout = useAppSelector((state) => state.site.layout);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (!site_layout) {
      // TODO
      dispatch(refresh({ title: "ttt", subhead: "sss", copyright: "ccc" }));
    }
  });
  return <span>&copy;{site_layout?.copyright}</span>;
};

export default Widget;
