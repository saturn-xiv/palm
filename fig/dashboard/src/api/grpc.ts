import { Metadata } from "grpc-web";

import { get as getToken } from "../reducers/current-user";
import { get as getLocale } from "../locales";

export const HOST: string =
  process.env.REACT_APP_GRPC_HOST || "http://127.0.0.1";

export const metadata = (): Metadata => {
  return {
    authorization: `Bearer ${getToken()}`,
    "accept-language": getLocale() || "en-US",
  };
};
