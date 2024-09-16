"use strict";

import { readFileSync } from "node:fs";
import { ServerCredentials } from "@grpc/grpc-js";

import logger from "./logger";

export class Config {
  constructor(file) {
    const raw = readFileSync(file);
    const it = JSON.parse(raw);
    this.port = it.port;
    // ServerCredentials.createInsecure(),
    this.credentials = ServerCredentials.createSsl(
      readFileSync(it.ssl["ca-file"]),
      [
        {
          cert_chain: readFileSync(it.ssl["cert-file"]),
          private_key: readFileSync(it.ssl["key-file"]),
        },
      ],
      false
    );
  }
}
