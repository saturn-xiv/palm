import { useState, useEffect } from "react";


import Show from "./Show";
import { index_ledger, ILedger } from "../../../api/daffodil";

const Widget = () => {
  const [items, setItems] = useState<ILedger[]>([]);
  useEffect(() => {
    index_ledger().then((res) => setItems(res));
  }, []);
  return (
    <>
      {items.map((it, id) => (
        <Show key={id} item={it} />
      ))}
    </>
  );
};

export default Widget;
