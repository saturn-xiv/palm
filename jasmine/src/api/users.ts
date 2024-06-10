import moment from "moment-timezone";

import { get as http_get, post as http_post } from ".";
import { home_url } from "../utils";
import { query, ISucceed } from "./graphql";

export interface IResource {
  type: string;
  id: number | null;
}
export interface IPermission {
  resource: IResource;
  action: string;
}

export const forgot_password_by_email = async (
  user: string
): Promise<Record<string, string>> => {
  return await http_post(`/api/users/by-email/forgot-password`, {
    user,
    home: home_url(),
  });
};
export const unlock_by_email = async (
  user: string
): Promise<Record<string, string>> => {
  return await http_post(`/api/users/by-email/unlock`, {
    user,
    home: home_url(),
  });
};
export const confirm_by_email = async (
  user: string
): Promise<Record<string, string>> => {
  return await http_post(`/api/users/by-email/confirm`, {
    user,
    home: home_url(),
  });
};

export const sign_up_by_email = async (
  realName: string,
  nickname: string,
  email: string,
  password: string
): Promise<ISucceed> => {
  const res = await query<{ signUpUserByEmail: ISucceed }>(
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
  return res.signUpUserByEmail;
};

export interface ICurrentUser {
  nickname: string;
  email: string;
  realName: string;
  avatar: string;
  isAdministrator: boolean;
  isRoot: boolean;
  roles: string[];
  permissions: IPermission[];
  hasWechatMiniProgram: boolean;
  hasWechatOauth2: boolean;
  hasGoogle: boolean;
  providerType: string;
  lang: string;
  timezone: string;
}

export interface ISignInResponse {
  token: string;
  user: ICurrentUser;
}

export const current_user = async (): Promise<ICurrentUser> => {
  return await http_get(`/api/users/current`);
};

export const sign_in_by_email = async (
  user: string,
  password: string
): Promise<ISignInResponse> => {
  return await http_post(`/api/users/current`, { user, password });
};
