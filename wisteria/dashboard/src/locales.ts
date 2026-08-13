import Cookies from "js-cookie";

import * as dayjs_ from "dayjs";
import AdvancedFormat from "dayjs/plugin/advancedFormat";
import IsLeapYear from "dayjs/plugin/isLeapYear";
import IsBetween from "dayjs/plugin/isBetween";
import IsToday from "dayjs/plugin/isToday";
import DayOfYear from "dayjs/plugin/dayOfYear";
import Duration from "dayjs/plugin/duration";
import Calendar from "dayjs/plugin/calendar";
import Timezone from "dayjs/plugin/timezone";
import Utc from "dayjs/plugin/utc";
import RelativeTime from "dayjs/plugin/relativeTime";
import "dayjs/locale/en";
import "dayjs/locale/zh-cn";
import "dayjs/locale/zh-tw";
import "dayjs/locale/ja";
import "dayjs/locale/ko";
import "dayjs/locale/my";

dayjs_.extend(AdvancedFormat);
dayjs_.extend(IsLeapYear);
dayjs_.extend(IsBetween);
dayjs_.extend(IsToday);
dayjs_.extend(DayOfYear);
dayjs_.extend(Duration);
dayjs_.extend(Calendar);
dayjs_.extend(Timezone);
dayjs_.extend(Utc);
dayjs_.extend(RelativeTime);

const KEY = "locale";

export const dayjs = (lang: string) => {
  switch (lang) {
    case "zh-Hans":
      dayjs_.locale("zh-cn");
      break;
    case "zh-Hant":
      dayjs_.locale("zh-tw");
      break;
    default:
      dayjs_.locale("en");
      break;
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
  import.meta.env.VITE_AVAILABLE_LANGUAGES || "en-US,zh-Hans"
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
