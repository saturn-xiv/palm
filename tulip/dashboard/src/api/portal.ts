import type { RpcError } from "grpc-web";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";

import { SiteClient } from "../protocols/PortalServiceClientPb";
import type { SiteHeartbeatResponse } from "../protocols/portal_pb";
import { BACKEND, metadata } from ".";

export const heartbeat = (
  callback: (err: RpcError, res: SiteHeartbeatResponse) => void,
) => {
  const req = new Empty();
  const cli = new SiteClient(BACKEND);
  cli.heartbeat(req, metadata(), (err, res) => {
    callback(err, res);
  });
};
