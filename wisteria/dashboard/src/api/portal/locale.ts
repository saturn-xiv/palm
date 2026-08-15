import graphql, { type Response as GraphqlResponse } from "../../graphql";
import { type IPagination } from ".";

interface IItem {
  id: number;
  code: string;
  message: string;
  updatedAt: Date;
}

interface IIndexResponse {
  indexLocale: {
    items: IItem[];
    pagination: IPagination;
  };
}

export const index = async (
  index: number,
  size: number,
): Promise<GraphqlResponse<IIndexResponse>> => {
  return graphql(
    `
      query call($page: Page!) {
        index(page: $page) {
          id
          lang
          code
          message
          updatedAt
        }
      }
    `,
    { page: { index, size } },
  );
};
interface IByLangResponse {
  localeByLang: { code: string; message: string }[];
}

export const by_lang = async (
  lang: string,
): Promise<GraphqlResponse<IByLangResponse>> => {
  return graphql(
    `
      query call($lang: String!) {
        localeByLang(lang: $lang) {
          code
          message
        }
      }
    `,
    { lang },
  );
};
