import { useState, useEffect } from "react";

import { PasswordBox } from "../protocols/portal_pb";

const hash = (payload: string): string => {
  const box = new PasswordBox();
  box.setPayload(payload);
  box.setSalt(Math.random());
  const buf: Uint8Array = box.serializeBinary();

  return buf.toBase64({
    alphabet: "base64url",
    omitPadding: true,
  });
};

const Widget = () => {
  const [password, setPassword] = useState<string>("");
  useEffect(() => {
    const load = async () => {
      const it = hash("palm");
      setPassword(it);
    };

    load();
  }, []);
  return <>Password: {password}</>;
};

export default Widget;
