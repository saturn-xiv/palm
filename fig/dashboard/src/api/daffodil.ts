import { query } from "../request";

const INDEX_LOCALE_BY_LANG = `
query call($lang: String!){
    indexLocaleByLang(lang: $lang) {
        code, message
    }
}
`;

export const index_locale_by_lang = async (
  lang: string
): Promise<{ code: string; message: string }[]> => {
  const res: {
    indexLocaleByLang: { code: string; message: string }[];
  } = await query(INDEX_LOCALE_BY_LANG, { lang });
  return res.indexLocaleByLang;
};
