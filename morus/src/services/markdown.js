"use strict";

import createDOMPurify from "dompurify";
import { JSDOM } from "jsdom";
import { parse as parse_markdown } from "marked";

import logger from "../logger";

export const to_html = (text, sanitize, result) => {
  logger.debug(`markdown to html(${sanitize}):\n${text}`);
  const html = parse_markdown(text);
  if (sanitize) {
    const window = new JSDOM("").window;
    const purify = createDOMPurify(window);
    const clean = purify.sanitize(html);
    result(null, clean);
  } else {
    result(null, html);
  }
};
