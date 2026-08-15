import { type ILayout as ISiteLayout } from "../site";

export interface ISignInResponse {
  token: string;
  user: ILayout;
  site: ISiteLayout;
}

export interface IRefreshResponse {
  user: ILayout;
  site: ISiteLayout;
}

export interface ILayout {
  lang: string;
  timezone: string;
  name: string;
  avatar: string;
  isAdministrator: boolean;
  roles: string[];
  permissions: IPermission[];
}

export interface IResource {
  type: string;
  id?: number;
}

export interface IPermission {
  action: string;
  resource: IResource;
}
