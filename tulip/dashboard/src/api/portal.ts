import type { RpcError } from "grpc-web";
import { Empty } from "google-protobuf/google/protobuf/empty_pb";

import { BACKEND, metadata } from ".";
import type { SiteHeartbeatResponse } from "grpc-web-client-gen/portal_pb";
import { SiteClient } from "grpc-web-client-gen/PortalServiceClientPb";

export const heartbeat = (
  callback: (err: RpcError, res: SiteHeartbeatResponse) => void,
) => {
  const req = new Empty();
  const cli = new SiteClient(BACKEND);
  cli.heartbeat(req, metadata(), (err, res) => {
    callback(err, res);
  });
};
