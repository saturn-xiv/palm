import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import timezone from "dayjs/plugin/timezone";
import isLeapYear from "dayjs/plugin/isLeapYear";
import localizedFormat from "dayjs/plugin/localizedFormat";
import advancedFormat from "dayjs/plugin/advancedFormat";

import "dayjs/locale/zh-cn";
import "dayjs/locale/zh-tw";
import "dayjs/locale/en";

dayjs.extend(utc);
dayjs.extend(isLeapYear);
dayjs.extend(timezone);
dayjs.extend(localizedFormat);
dayjs.extend(advancedFormat);

export const home_url = (): string => {
  const url = new URL("/", window.location.href);
  return url.toString();
};

export const timezones = (): string[] => {
  return Intl.supportedValuesOf("timeZone");
};

export const guess_timezone = (): string => {
  return dayjs.tz.guess();
};
