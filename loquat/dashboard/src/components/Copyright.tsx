import { useEffect } from "react";

import { useAppSelector, useAppDispatch } from "../hooks";
import { refresh } from "../reducers/layout";
import { refresh as refresh_layout } from "../api";

const Widget = () => {
  const layout = useAppSelector((state) => state.layout);
  const dispatch = useAppDispatch();
  useEffect(() => {
    if (layout.version === undefined) {
      const handle_refresh = async () => {
        const res = await refresh_layout();
        if (res.data) {
          dispatch(
            refresh({
              hostname: res.data.refresh.hostname,
              version: res.data.refresh.version,
            })
          );
        }
      };
      handle_refresh();
    }
  }, [layout, dispatch]);
  return (
    <div className="content has-text-centered">
      <strong>{layout.hostname}</strong>&nbsp;
      <span>
        &copy;{new Date().getFullYear()}&nbsp;{layout.version}.
      </span>
      &nbsp;
      <span>
        The source code is licensed &nbsp;
        <a target="_blank" href="https://opensource.org/license/mit">
          MIT
        </a>
        .
      </span>
    </div>
  );
};

export default Widget;
