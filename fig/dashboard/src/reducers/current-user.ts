const KEY = "token";
export const DURATION = 60 * 60 * 24;

export const SIGN_IN_PATH = "/anonymous/users/sign-in";

const ENABLE_LOCAL_TOKEN = import.meta.env.VITE_ENABLE_LOCAL_TOKEN === "true";

export const get = (): string | null => {
  const token = sessionStorage.getItem(KEY);
  if (token) {
    return token;
  }
  if (ENABLE_LOCAL_TOKEN) {
    return localStorage.getItem(KEY);
  }
  return null;
};

export const set = (token: string) => {
  sessionStorage.setItem(KEY, token);
  if (ENABLE_LOCAL_TOKEN) {
    localStorage.setItem(KEY, token);
  }
};

export const remove = () => {
  sessionStorage.removeItem(KEY);
  localStorage.removeItem(KEY);
};
