import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import timezone from "dayjs/plugin/timezone";
import isLeapYear from "dayjs/plugin/isLeapYear";
import "dayjs/locale/zh-cn";
import "dayjs/locale/zh-tw";
import "dayjs/locale/en";
import moment from "moment-timezone";

dayjs.extend(utc);
dayjs.extend(isLeapYear);
dayjs.extend(timezone);

export const home_url = (): string => {
  const url = new URL("/", window.location.href);
  return url.toString();
};

export const timezones = (): string[] => {
  return moment.tz.names();
};

export const guess_timezone = (): string => {
  return dayjs.tz.guess();
};
