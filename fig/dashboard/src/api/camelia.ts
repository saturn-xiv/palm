import { query } from "./graphql";

export interface ISucceed {
  createdAt: Date;
}

export interface IAuthor {
  name: string;
  email: string;
}

interface ISiteInfoResponse {
  title: string;
  subhead: string;
  description: string;
  keywords: string[];
  languages: string[];
  authors: IAuthor[];
}
export interface ILayoutResponse {
  apiVersion: string;
  siteInfo: ISiteInfoResponse;
}
export const fetch_layout = async (lang: string): Promise<ILayoutResponse> => {
  const res = await query<ILayoutResponse>(
    `
query call($lang: String!){
  apiVersion
  siteInfo(lang: $lang){
    title, subhead, description, keywords, languages, 
    authors{name, email}
  }
}
`,
    { lang }
  );
  return res;
};

interface IIndexLocaleResponse {
  indexLocaleByLang: { code: string; message: string }[];
}

export const index_locale = async (
  lang: string
): Promise<Record<string, string>> => {
  const res = await query<IIndexLocaleResponse>(
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
