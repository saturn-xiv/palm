import { useEffect } from "react";

import { useAppDispatch } from "../../hooks";

const Widget = () => {
  const dispatch = useAppDispatch();

  useEffect(() => {
    (async () => {
      // TODO
    })();
  }, [dispatch]);

  return <div className="grid"></div>;
};

export default Widget;
