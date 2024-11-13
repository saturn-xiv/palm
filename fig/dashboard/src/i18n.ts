import Cookies from "js-cookie";
import enUS from "antd/locale/en_US";
import zhCN from "antd/locale/zh_CN";
import zhTW from "antd/locale/zh_TW";
import dayjs from "dayjs";
import type { ConfigProviderProps } from "antd";

import "dayjs/locale/zh-cn";

const KEY = "locale";

export type Locale = ConfigProviderProps["locale"];

export const load = (lang: string): Locale => {
  switch (lang) {
    case "zh-Hans":
      dayjs.locale("zh-cn");
      return zhCN;
    case "zh-Hant":
      dayjs.locale("zh-tw");
      return zhTW;
    default:
      dayjs.locale("en");
      return enUS;
  }
};

export const get = (): string => {
  return (
    localStorage.getItem(KEY) || Cookies.get(KEY) || DEFAULT_LANGUAGE || "en-US"
  );
};

export const available_languages: string[] = (
  import.meta.env.VITE_AVAILABLE_LANGUAGES || "en-US,zh-Hans,zh-Hant"
).split(",");

export const set = (lang: string, reload: boolean) => {
  Cookies.set(KEY, lang);
  localStorage.setItem(KEY, lang);
  if (reload) {
    window.location.reload();
  }
};

export const remove = () => {
  Cookies.remove(KEY);
  localStorage.removeItem(KEY);
};

export const DEFAULT_LANGUAGE = import.meta.env.VITE_DEFAULT_LANGUAGE;
