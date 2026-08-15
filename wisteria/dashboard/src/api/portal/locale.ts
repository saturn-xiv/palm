import graphql from "../../graphql";
import { type IPagination, type ISucceeded } from ".";

export const set = async (
  lang: string,
  code: string,
  message: string,
): Promise<ISucceeded> => {
  const res: { setLocale: ISucceeded } = await graphql(
    `
      mutation call($lang: String!, $code: String!, $message: String!) {
        setLocale(lang: $lang, code: $code, message: $message) {
          createdAt
        }
      }
    `,
    { lang, code, message },
  );
  return res.setLocale;
};

export const destroy = async (id: number): Promise<ISucceeded> => {
  const res: { destroyLocale: ISucceeded } = await graphql(
    `
      mutation call($id: Int!) {
        destroyLocale(id: $id) {
          createdAt
        }
      }
    `,
    { id },
  );
  return res.destroyLocale;
};

interface IItem {
  id: number;
  code: string;
  message: string;
  updatedAt: Date;
}

interface IIndexResponse {
  items: IItem[];
  pagination: IPagination;
}

export const index = async (
  index: number,
  size: number,
): Promise<IIndexResponse> => {
  const res: { indexLocale: IIndexResponse } = await graphql(
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
  return res.indexLocale;
};

export const by_lang = async (
  lang: string,
): Promise<{ code: string; message: string }[]> => {
  const res: {
    localeByLang: { code: string; message: string }[];
  } = await graphql(
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
  return res.localeByLang;
};
