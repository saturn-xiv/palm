import { useEffect, useState } from "react";

import {
  indexNetworkInterface,
  type INetworkInterface,
} from "../../api/router";
import { useAppDispatch } from "../../hooks";
import { danger as show_danger } from "../../reducers/notification";
import ShowNetworkInterface from "./NetworkInterface";

const Widget = () => {
  const dispatch = useAppDispatch();
  const [items, setItems] = useState<INetworkInterface[]>([]);
  useEffect(() => {
    (async () => {
      const res = await indexNetworkInterface();
      if (res.data?.indexNetworkInterface) {
        setItems(res.data.indexNetworkInterface);
      } else if (res.errors) {
        dispatch(show_danger(res.errors));
      }
    })();
  }, [dispatch]);

  return (
    <div className="grid">
      {items.map((it, id) => (
        <ShowNetworkInterface key={id} item={it} />
      ))}
    </div>
  );
};

export default Widget;
