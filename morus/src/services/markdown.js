"use strict";

import createDOMPurify from "dompurify";
import { JSDOM } from "jsdom";
import { parse as parse_markdown } from "marked";

import { MarkdownToHtmlResponse } from "../protocols/morus_pb";
import logger from "../logger";

export const to_html = (call, callback) => {
  logger.debug(
    `markdown to html(${call.request.getSanitize()}): ${call.request.getPayload()}`
  );
  const html = parse_markdown(call.request.getPayload());
  var reply = new MarkdownToHtmlResponse();
  if (call.request.getSanitize()) {
    const window = new JSDOM("").window;
    const purify = createDOMPurify(window);
    const clean = purify.sanitize(html);
    reply.setPayload(clean);
  } else {
    reply.setPayload(html);
  }
  callback(null, reply);
};
