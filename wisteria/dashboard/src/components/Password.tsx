import { useState, useEffect } from "react";
import * as flatbuffers from "flatbuffers";

import { Password } from "../protocols/palm/portal/v1";

// const sha512_hash = async (plain: string): Promise<string> => {
//   const builder = new flatbuffers.Builder(1 << 10);

//   const encoder = new TextEncoder();
//   const data = encoder.encode(plain);
//   const buffer = await crypto.subtle.digest("SHA-512", data);
//   const hash = builder.createByteVector(new Uint8Array(buffer));

//   Password.startPassword(builder);
//   Password.addHash(builder, hash);
//   Password.addSalt(builder, Math.random());
//   const offset = Password.endPassword(builder);
//   builder.finish(offset);

//   const tmp: Uint8Array = builder.asUint8Array();
//   return (tmp as any).toBase64({ omitPadding: true, alphabet: "base64url" });
// };

const hash = (payload: string): string => {
  const builder = new flatbuffers.Builder(1 << 10);

  const payload_ = builder.createString(payload);
  Password.startPassword(builder);
  Password.addPayload(builder, payload_);
  Password.addSalt(builder, Math.random());
  const offset = Password.endPassword(builder);
  builder.finish(offset);

  const tmp: Uint8Array = builder.asUint8Array();
  return (tmp as any).toBase64({ omitPadding: true, alphabet: "base64url" });
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
