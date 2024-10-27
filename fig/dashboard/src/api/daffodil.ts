import { get as detect_locale } from "../i18n";
import { query, ISucceed } from ".";

export interface ISetSiteInfoRequest {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
}
export interface IUserSignUpByEmailRequest {
  email: string;
  realName: string;
  nickname: string;
  password: string;
  timezone: string;
}

const INSTALL = `
mutation call($lang: String!, $site: SetSiteInfoRequest!, $user: UserSignUpByEmailRequest!){
    install(lang: $lang, site: $site, user: $user) {
        createdAt
    }
}
`;

export const install = async (
  site: ISetSiteInfoRequest,
  user: IUserSignUpByEmailRequest
): Promise<ISucceed> => {
  const res: ISucceed = await query(INSTALL, {
    lang: detect_locale(),
    site,
    user,
  });
  return res;
};

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
