import { get as detect_locale } from "../i18n";
import { query, ISucceed } from ".";

export interface ISetSiteInfoRequest {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
}

const USER_RESET_PASSWORD_BY_TOKEN = `
mutation call($token: String!, $password: String!){
    userResetPasswordByToken(token: $token, password: $password) {
        createdAt
    }
}
`;
export const user_reset_password_by_token = async (
  token: string,
  password: string
): Promise<ISucceed> => {
  const res: ISucceed = await query(USER_RESET_PASSWORD_BY_TOKEN, {
    token,
    password,
  });
  return res;
};
const USER_FORGOT_PASSWORD_BY_EMAIL = `
mutation call($user: String!){
    userForgotPasswordByEmail(user: $user) {
        createdAt
    }
}
`;
export const user_forgot_password_by_email = async (
  user: string
): Promise<ISucceed> => {
  const res: ISucceed = await query(USER_FORGOT_PASSWORD_BY_EMAIL, {
    user,
  });
  return res;
};
const USER_UNLOCK_BY_TOKEN = `
mutation call($token: String!){
    userUnlockByToken(token: $token) {
        createdAt
    }
}
`;
export const user_unlock_by_token = async (
  token: string
): Promise<ISucceed> => {
  const res: ISucceed = await query(USER_UNLOCK_BY_TOKEN, {
    token,
  });
  return res;
};
const USER_UNLOCK_BY_EMAIL = `
mutation call($user: String!){
    userUnlockByEmail(user: $user) {
        createdAt
    }
}
`;
export const user_unlock_by_email = async (user: string): Promise<ISucceed> => {
  const res: ISucceed = await query(USER_UNLOCK_BY_EMAIL, {
    user,
  });
  return res;
};
const USER_CONFIRM_BY_TOKEN = `
mutation call($token: String!){
    userConfirmByToken(token: $token) {
        createdAt
    }
}
`;
export const user_confirm_by_token = async (
  token: string
): Promise<ISucceed> => {
  const res: ISucceed = await query(USER_CONFIRM_BY_TOKEN, {
    token,
  });
  return res;
};
const USER_CONFIRM_BY_EMAIL = `
mutation call($user: String!){
    userConfirmByEmail(user: $user) {
        createdAt
    }
}
`;
export const user_confirm_by_email = async (
  user: string
): Promise<ISucceed> => {
  const res: ISucceed = await query(USER_CONFIRM_BY_EMAIL, {
    user,
  });
  return res;
};

export interface ISignInResponse {
  token: string;
}

const USER_SIGN_IN_BY_EMAIL = `
mutation call($user: String!, $password: String!){
    userSignInByEmail(user: $user, password: $password) {
        createdAt
    }
}
`;
export const user_sign_in_by_email = async (
  user: string,
  password: string
): Promise<ISignInResponse> => {
  const res: ISignInResponse = await query(USER_SIGN_IN_BY_EMAIL, {
    user,
    password,
  });
  return res;
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
  const res: ISucceed = await query(USER_SIGN_UP_BY_EMAIL, {
    lang: detect_locale(),
    form,
  });
  return res;
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
