import Cookies from "js-cookie";

import antdZhCN from "antd/locale/zh_CN";
import antdZhTW from "antd/locale/zh_TW";
import antdEnUS from "antd/locale/en_US";
import { Locale as AntdLocale } from "antd/lib/locale";

const KEY = "locale";

export interface IItem {
  id: number;
  code: string;
  message: string;
  updatedAt: Date;
}

export const antd = (lang: string): AntdLocale => {
  switch (lang) {
    case "zh-Hans":
      return antdZhCN;
    case "zh-Hant":
      return antdZhTW;
    default:
      return antdEnUS;
  }
};

export const get = (): string => {
  return (
    localStorage.getItem(KEY) ||
    Cookies.get(KEY) ||
    import.meta.env.VITE_DEFAULT_LOCALE ||
    "en-US"
  );
};

export const available_languages: string[] = (
  import.meta.env.VITE_APP_AVAILABLE_LANGUAGES || "en-US,zh-Hans,zh-Hant"
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

export interface ILocale {
  messages: Record<string, string>;
}
