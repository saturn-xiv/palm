import Cookies from "js-cookie";
import { type MessageFormatElement } from "react-intl";

import enUS from "./en-US";
import zhHans from "./zh-Hans";
import zhHant from "./zh-Hant";

const KEY = "locale";

export const detect = (): string => Cookies.get(KEY) || "en-US";

export const set = (locale: string) => {
  Cookies.set(KEY, locale);
};

export const messages = (
  locale: string
): Record<string, string> | Record<string, MessageFormatElement[]> => {
  switch (locale) {
    case "zh-Hans":
      return zhHans;
    case "zh-Hants":
      return zhHant;
    default:
      return enUS;
  }
};
