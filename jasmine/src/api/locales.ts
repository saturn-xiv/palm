import { query, IPagination, ISucceed } from "./graphql";

export interface ILocale {
  id: number;
  lang: string;
  code: string;
  message: string;
  updatedAt: Date;
}

interface IIndexLocaleByLangResponse {
  indexLocaleByLang: { code: string; message: string }[];
}

export const intl_messages_by_lang = async (
  lang: string
): Promise<Record<string, string>> => {
  const res = await query<IIndexLocaleByLangResponse>(
    `
query call($lang: String!){
  indexLocaleByLang(lang: $lang){
    code, message
  }
}
`,
    { lang }
  );

  const messages = res.indexLocaleByLang.reduce(
    (ac, it) => ({ ...ac, [it.code]: it.message }),
    {}
  );
  return messages;
};

interface IIndexLocaleResponse {
  items: ILocale[];
  pagination: IPagination;
}
export const index = async (
  page: number,
  size: number
): Promise<IIndexLocaleResponse> => {
  const res = await query<{ indexLocale: IIndexLocaleResponse }>(
    `
query call($pager: Pager!){
  indexLocale(pager: $pager){
    items{id, lang, code, message, updatedAt},
    pagination{page, size, total, hasNext, hasPrevious}
  }
}
`,
    {
      pager: { page, size },
    }
  );
  return res.indexLocale;
};

export const save = async (
  lang: string,
  code: string,
  message: string
): Promise<ISucceed> => {
  const res = await query<{ setLocale: ISucceed }>(
    `
mutation call($lang: String!, $code: String!, $message: String!){
  setLocale(lang: $lang, code: $code, message: $message){
    createdAt
  }
}
`,
    { lang, code, message }
  );
  return res.setLocale;
};

export const destroy = async (id: number): Promise<ISucceed> => {
  const res = await query<{ destroyLocale: ISucceed }>(
    `
mutation call($id: ID!){
  destroyLocale(id: $id){
    createdAt
  }
}
`,
    { id }
  );
  return res.destroyLocale;
};
