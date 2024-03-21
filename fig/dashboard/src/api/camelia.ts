import moment from "moment-timezone";

import { query } from "./graphql";
import { home_url } from "../utils";
import { IPermission } from "../reducers/current-user";

export const EDITOR_TEXTAREA = "TEXTAREA";

export interface ISucceed {
  createdAt: Date;
}

export interface ISignInResponse {
  token: string;
  realName: string;
  isAdministrator: boolean;
  isRoot: boolean;
  roles: string[];
  permissions: IPermission[];
  hasWechatMiniProgram: boolean;
  hasWechatOauth2: boolean;
  hasGoogle: boolean;
  providerType: string;
}

export const sign_in_by_email = async (
  user: string,
  password: string
): Promise<ISignInResponse> => {
  const res = await query<{ signInUserByEmail: ISignInResponse }>(
    `
mutation call($user: String!, $password: String!){
  signInUserByEmail(user: $user, password: $password){    
    token, realName, providerType,
    isAdministrator, isRoot,
    roles, 
    permissions{ 
      resource{type, sid, iid},
      action
    },
    hasWechatMiniProgram, hasWechatOauth2, hasGoogle    
  }
}
`,
    {
      user,
      password,
      ttl: 60 * 60 * 24,
    }
  );
  return res.signInUserByEmail;
};

export const reset_password = async (
  token: string,
  password: string
): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($token: String!, $password: String!){
  resetUserPassword(token: $token, password: $password){
    createdAt
  }
}
`,
    {
      token,
      password,
    }
  );
  return res;
};
export const forgot_password = async (user: string): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($user: String!, $home: String!){
  forgotUserPassword(user: $user, home: $home){
    createdAt
  }
}
`,
    {
      user,
      home: home_url(),
    }
  );
  return res;
};
export const unlock_by_email = async (user: string): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($user: String!, $home: String!){
  unlockUserByEmail(user: $user, home: $home){
    createdAt
  }
}
`,
    {
      user,
      home: home_url(),
    }
  );
  return res;
};

export const unlock_by_token = async (token: string): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($token: String!){
  unlockUserByToken(token: $token){
    createdAt
  }
}
`,
    {
      token,
    }
  );
  return res;
};
export const confirm_by_email = async (user: string): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($user: String!, $home: String!){
  confirmUserByEmail(user: $user, home: $home){
    createdAt
  }
}
`,
    {
      user,
      home: home_url(),
    }
  );
  return res;
};

export const confirm_by_token = async (token: string): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($token: String!){
  confirmUserByToken(token: $token){
    createdAt
  }
}
`,
    {
      token,
    }
  );
  return res;
};
export const sign_up_by_email = async (
  realName: string,
  nickname: string,
  email: string,
  password: string
): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($realName: String!, $nickname: String!, $email: String!, $password: String!, $home: String!, $timezone: String!){
  signUpUserByEmail(realName: $realName, nickname: $nickname, email: $email, password: $password, home: $home, timezone: $timezone){
    createdAt
  }
}
`,
    {
      realName,
      nickname,
      email,
      password,
      home: home_url(),
      timezone: moment.tz.guess(),
    }
  );
  return res;
};

export const create_leave_word = async (
  content: string,
  editor: string
): Promise<ISucceed> => {
  const res = await query<ISucceed>(
    `
mutation call($content: String!, $editor: MediaTextEditor!){
  createLeaveWord(content: $content, editor: $editor){
    createdAt
  }
}
`,
    { content, editor }
  );
  return res;
};

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
