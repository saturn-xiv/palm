"use strict";

import fs from "fs";
import path from "path";

import {
  createMultiplexServer,
  MultiplexedProcessor,
  TFramedTransport,
  TBinaryProtocol,
} from "thrift";
import Mustache from "mustache";

import { Config } from "./env";
import logger from "./logger";
import { to_html as markdown_to_html } from "./services/markdown";
import { Processor as MarkdownProcessor } from "./protocols/Markdown";

function generate_systemd_conf(file) {
  const systemd = "morus.config";
  logger.info(`generate ${systemd}`);
  const buf = Mustache.render(
    `
[Unit]
Description=A markdown-to-html service.
Wants=network-online.target
After=network-online.target

[Service]
Type=notify
User=root
Group=root
ExecStart=/usr/bin/node {{ file }} rpc config.json
WorkingDirectory=/var/lib/morus
Restart=always

[Install]
WantedBy=multi-user.target
`,
    { file: path.basename(file) }
  );
  fs.writeFileSync(systemd, buf);
}

function set_logger(config) {
  if (config.debug) {
    logger.level = "debug";
    logger.debug("run on debug mode");
  }
}

function launch_rpc_server(config) {
  var processor = new MultiplexedProcessor();
  {
    const name = "markdown";
    logger.info(`register markdown service(${name})`);
    var service = new MarkdownProcessor({ to_html: markdown_to_html });
    processor.registerProcessor(name, service);
  }

  var options = {
    transport: TFramedTransport,
    protocol: TBinaryProtocol,
  };

  const server = createMultiplexServer(processor, options);
  logger.info(`start rpc server on tcp://0.0.0.0:${config.port}`);
  server.listen(config.port);
}

function main() {
  const args = process.argv;
  if (args.length === 3 && args[2] === "systemd") {
    generate_systemd_conf(args[1]);
  } else if (args.length === 4 && args[2] === "rpc") {
    const config = new Config(args[3]);
    set_logger(config);
    launch_rpc_server(config);
  } else {
    logger.error(
      `USAGE:\nnode ${args[1]} rpc CONFIG_FILE\nnode ${args[1]} systemd`
    );
    return;
  }
}

main();
