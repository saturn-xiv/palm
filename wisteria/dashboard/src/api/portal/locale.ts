import graphql, { type Response as GraphqlResponse } from "../../graphql";

export interface IItem {
  id: number;
  code: string;
  message: string;
  updatedAt: Date;
}

export interface IIndexResponse {
  localeByLang: IItem[];
}

export const by_lang = async (
  lang: string,
): Promise<GraphqlResponse<IIndexResponse>> => {
  return graphql(
    `
      query call($lang: String!) {
        localeByLang(lang: $lang) {
          id
          lang
          code
          message
          updatedAt
        }
      }
    `,
    { lang },
  );
};
