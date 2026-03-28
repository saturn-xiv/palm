import { useEffect, useState } from "react";
import { heartbeat } from "./api/portal";

const Widget = () => {
  const [version, setVersion] = useState<string>();

  useEffect(() => {
    heartbeat((err, res) => {
      if (err) {
        console.log(err.code, err.message);
        return;
      }
      setVersion(res.getVersion());
    });
  }, []);

  return <div>Version: {version}</div>;
};

export default Widget;
