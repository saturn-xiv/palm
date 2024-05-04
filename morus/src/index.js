"use strict";

import {
  createMultiplexServer,
  MultiplexedProcessor,
  TFramedTransport,
  TBinaryProtocol,
} from "thrift";

import { Config } from "./env";
import logger from "./logger";
import { to_html as markdown_to_html } from "./services/markdown";
import { Processor as MarkdownProcessor } from "./protocols/Markdown";

function main() {
  const args = process.argv;
  if (args.length !== 3) {
    logger.error(`USAGE: node ${args[1]} CONFIG_FILE`);
    return;
  }
  const config = new Config("config.json");
  if (config.debug) {
    logger.level = "debug";
    logger.debug("run on debug mode");
  }

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

main();
