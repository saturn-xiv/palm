import { query } from "./graphql";

export interface ISucceed {
  createdAt: Date;
}

interface IIndexLocaleResponse {
  indexLocaleByLang: { code: string; message: string }[];
}

export const index_locale = async (
  lang: string
): Promise<Record<string, string>> => {
  const res = await query<IIndexLocaleResponse>(`
query {
  indexLocaleByLang(lang:"${lang}"){
    code, message
  }
}
`);
  if ("data" in res) {
    const messages = res.data.indexLocaleByLang.reduce(
      (ac, it) => ({ ...ac, [it.code]: it.message }),
      {}
    );
    return messages;
  }
  return {};
};
