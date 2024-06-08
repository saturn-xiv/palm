import { get as http_get, post as http_post } from ".";

export interface IResource {
  type: string;
  iid: number | null;
  sid: string | null;
}
export interface IPermission {
  resource: IResource;
  action: string;
}

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
