import { get as detect_locale } from "../i18n";
import { query, ISucceed, EDITOR_TEXTAREA } from ".";
import { ICurrentUser } from "../reducers/current-user";
import { ISiteInfo } from "../reducers/site";

interface IRefreshResponse {
  currentUser?: ICurrentUser;
  siteInfo: ISiteInfo;
}

const REFRESH = `
query call{
    refresh{
        siteInfo{
          favicon, title, subhead, keywords, description, copyright, locale, languages,
          author{name, email},
          cnBi{code}, cnIcp{code}, cnGab{code}
        }
        currentUser{
          realName, providerType, lang, timezone, isAdministrator, isRoot, roles
          permissions {operation, resource {type, id}},
          sideBar{
            label, to, external,
            children{label, to, external}
          }
        }
    }
}
`;
export const refresh = async (): Promise<IRefreshResponse> => {
  const res: { refresh: IRefreshResponse } = await query(REFRESH, {});
  return res.refresh;
};

const USER_SIGN_OUT = `
mutation call{
    userSignOut {
        createdAt
    }
}
`;
export const user_sign_out = async (): Promise<ISucceed> => {
  const res: { userSignOut: ISucceed } = await query(USER_SIGN_OUT, {});
  return res.userSignOut;
};

export interface ISetSiteInfoRequest {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
}

const CREATE_LEAVE_WORD = `
mutation call($body: String!, $editor: Editor!){
    createLeaveWord(body: $body, editor: $editor) {
        createdAt
    }
}
`;
export const create_leave_word = async (body: string): Promise<ISucceed> => {
  const res: { createLeaveWord: ISucceed } = await query(CREATE_LEAVE_WORD, {
    body,
    editor: EDITOR_TEXTAREA,
  });
  return res.createLeaveWord;
};
const RESET_EMAIL_USER_PASSWORD_BY_TOKEN = `
mutation call($token: String!, $password: String!){
    resetEmailUserPasswordByToken(token: $token, password: $password) {
        createdAt
    }
}
`;
export const reset_email_user_password_by_token = async (
  token: string,
  password: string
): Promise<ISucceed> => {
  const res: { resetEmailUserPasswordByToken: ISucceed } = await query(
    RESET_EMAIL_USER_PASSWORD_BY_TOKEN,
    {
      token,
      password,
    }
  );
  return res.resetEmailUserPasswordByToken;
};
const SEND_FORGOT_PASSWORD_EMAIL_FOR_USER = `
query call($user: String!){
    sendForgotPasswordEmailForUser(user: $user) {
        createdAt
    }
}
`;
export const send_forgot_password_email_for_user = async (
  user: string
): Promise<ISucceed> => {
  const res: { sendForgotPasswordEmailForUser: ISucceed } = await query(
    SEND_FORGOT_PASSWORD_EMAIL_FOR_USER,
    {
      user,
    }
  );
  return res.sendForgotPasswordEmailForUser;
};
const UNLOCK_EMAIL_USER_BY_TOKEN = `
mutation call($token: String!){
    unlockEmailUserByToken(token: $token) {
        createdAt
    }
}
`;
export const unlock_email_user_by_token = async (
  token: string
): Promise<ISucceed> => {
  const res: { unlockEmailUserByToken: ISucceed } = await query(
    UNLOCK_EMAIL_USER_BY_TOKEN,
    {
      token,
    }
  );
  return res.unlockEmailUserByToken;
};
const SEND_UNLOCK_EMAIL_FOR_USER = `
query call($user: String!){
    sendUnlockEmailForUser(user: $user) {
        createdAt
    }
}
`;
export const send_unlock_email_for_user = async (
  user: string
): Promise<ISucceed> => {
  const res: { sendUnlockEmailForUser: ISucceed } = await query(
    SEND_UNLOCK_EMAIL_FOR_USER,
    {
      user,
    }
  );
  return res.sendUnlockEmailForUser;
};
const CONFIRM_EMAIL_USER_BY_TOKEN = `
mutation call($token: String!){
    confirmEmailUserByToken(token: $token) {
        createdAt
    }
}
`;
export const confirm_email_user_by_token = async (
  token: string
): Promise<ISucceed> => {
  const res: { confirmEmailUserByToken: ISucceed } = await query(
    CONFIRM_EMAIL_USER_BY_TOKEN,
    {
      token,
    }
  );
  return res.confirmEmailUserByToken;
};
const SEND_CONFIRM_EMAIL_FOR_USER = `
query call($user: String!){
    sendConfirmEmailForUser(user: $user) {
        createdAt
    }
}
`;
export const send_confirm_email_for_user = async (
  user: string
): Promise<ISucceed> => {
  const res: { sendConfirmEmailForUser: ISucceed } = await query(
    SEND_CONFIRM_EMAIL_FOR_USER,
    {
      user,
    }
  );
  return res.sendConfirmEmailForUser;
};

const USER_SIGN_IN_BY_EMAIL = `
mutation call($user: String!, $password: String!){
    userSignInByEmail(user: $user, password: $password) {
        token,
        profile{
          realName, providerType, lang, timezone, isAdministrator, isRoot, roles
          permissions {operation, resource {type, id}},
          sideBar{
            label, to, external,
            children{label, to, external}
          }
        }
    }
}
`;

interface IUserSignInResponse {
  token: string;
  profile: ICurrentUser;
}
export const user_sign_in_by_email = async (
  user: string,
  password: string
): Promise<IUserSignInResponse> => {
  const res: { userSignInByEmail: IUserSignInResponse } = await query(
    USER_SIGN_IN_BY_EMAIL,
    {
      user,
      password,
    }
  );
  return res.userSignInByEmail;
};

export interface IUserSignUpByEmailRequest {
  email: string;
  realName: string;
  nickname: string;
  password: string;
  timezone: string;
}

const USER_SIGN_UP_BY_EMAIL = `
mutation call($lang: String!, $form: UserSignUpByEmailRequest!){
    userSignUpByEmail(lang: $lang, form: $form) {
        createdAt
    }
}
`;

export const user_sign_up_by_email = async (
  form: IUserSignUpByEmailRequest
): Promise<ISucceed> => {
  const res: { userSignUpByEmail: ISucceed } = await query(
    USER_SIGN_UP_BY_EMAIL,
    {
      lang: detect_locale(),
      form,
    }
  );
  return res.userSignUpByEmail;
};

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
  const res: { install: ISucceed } = await query(INSTALL, {
    lang: detect_locale(),
    site,
    user,
  });
  return res.install;
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
