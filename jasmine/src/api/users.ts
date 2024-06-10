import moment from "moment-timezone";

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
): Promise<ISucceed> => {
  const res = await query<{ forgotUserPasswordByEmail: ISucceed }>(
    `
mutation call($user: String!, $home: String!){
  forgotUserPasswordByEmail(user: $user, home: $home){
    createdAt
  }
}
`,
    {
      user,
      home: home_url(),
    }
  );
  return res.forgotUserPasswordByEmail;
};
export const unlock_by_email = async (user: string): Promise<ISucceed> => {
  const res = await query<{ unlockUserByEmail: ISucceed }>(
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
  return res.unlockUserByEmail;
};
export const confirm_by_email = async (user: string): Promise<ISucceed> => {
  const res = await query<{ confirmUserByEmail: ISucceed }>(
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
  return res.confirmUserByEmail;
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

// ----------------------------------------------------------------------------

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
  const res = await query<{ currentUser: ICurrentUser }>(
    `
query call{
  currentUser{
    nickname, email, realName, avatar, providerType, lang, timezone,
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
    {}
  );
  return res.currentUser;
};

export const sign_in_by_email = async (
  user: string,
  password: string
): Promise<ISignInResponse> => {
  const res = await query<{ signInUserByEmail: ISignInResponse }>(
    `
mutation call($user: String!, $password: String!){
  signInUserByEmail(user: $user, password: $password){    
    token, 
    user{
      realName, avatar, providerType, lang, timezone,
      isAdministrator, isRoot,
      roles, 
      permissions{ 
        resource{type, sid, iid},
        action
      },
      hasWechatMiniProgram, hasWechatOauth2, hasGoogle
    }
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
