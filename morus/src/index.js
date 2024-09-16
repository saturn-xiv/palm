"use strict";

import { Server } from "@grpc/grpc-js";
import { HealthImplementation } from "grpc-health-check";

import { Config } from "./env";
import logger from "./logger";
import { MarkdownService } from "./protocols/morus_grpc_pb";
import { to_html } from "./services/markdown";
import { status_map } from "./services/status";

function main() {
  const args = process.argv;
  if (args.length !== 3) {
    logger.error(`USAGE: node ${args[1]} CONFIG_FILE`);
    return;
  }
  const config = new Config("config.json");
  const health = new HealthImplementation(status_map);

  var server = new Server();
  server.addService(MarkdownService, { toHtml: to_html });
  health.addToServer(server);

  logger.info(`start gRPC server on http://0.0.0.0:${config.port}`);
  server.bindAsync(`0.0.0.0:${config.port}`, config.credentials, () => {});
}

main();
