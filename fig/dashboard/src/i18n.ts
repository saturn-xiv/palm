import Cookies from "js-cookie";

const KEY = "locale";

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
