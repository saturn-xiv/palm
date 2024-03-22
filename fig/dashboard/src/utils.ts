import moment from "moment";

export const home_url = (): string => {
  const url = new URL("/", window.location.href);
  return url.toString();
};

const DATETIME_FORMAT = "llll";
export const date_to_str = (it: Date): string => {
  return moment(it).format(DATETIME_FORMAT);
};

export const timestamp_to_str = (it: number): string => {
  return moment.unix(it).format(DATETIME_FORMAT);
};
