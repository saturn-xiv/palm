import { PasswordBox } from "../protocols/portal_pb";

export const password = (payload: string): string => {
  const box = new PasswordBox();
  box.setPayload(payload);
  box.setSalt(Math.random());
  const buf: Uint8Array = box.serializeBinary();

  return buf.toBase64({
    alphabet: "base64url",
    omitPadding: true,
  });
};
