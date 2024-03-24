export const home_url = (): string => {
  const url = new URL("/", window.location.href);
  return url.toString();
};
