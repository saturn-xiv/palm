import { get as http_get } from "./";

interface ILocale {
  id: number;
  lang: string;
  code: string;
  message: string;
  updatedAt: Date;
}

export const intl_messages_by_lang = async (
  lang: string
): Promise<Record<string, string>> => {
  const items: ILocale[] = await http_get(`/api/locales/by-lang/${lang}`);
  return items.reduce((ac, it) => ({ ...ac, [it.code]: it.message }), {});
};

export const all = async (): Promise<ILocale[]> => {
  // TODO
  const items: ILocale[] = await http_get("/api/locales");
  return items;
};
