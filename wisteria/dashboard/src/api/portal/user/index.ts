import { type ILayout as ISiteLayout } from "../site";
import graphql from "../../../graphql";

export interface ISignInResponse {
  token: string;
  user: ILayout;
  site: ISiteLayout;
}

export interface IRefreshResponse {
  user: ILayout;
  site: ISiteLayout;
}

export const refresh = async (): Promise<IRefreshResponse> => {
  const res: { refresh: IRefreshResponse } = await graphql(
    `
      query call {
        refresh {
          user {
            lang
            timezone
            name
            avatar
            isAdministrator
            roles
            permissions {
              action
              resource {
                type
                id
              }
            }
          }
          site {
            favicon
            title
            subhead
            author {
              name
              email
            }
            keywords
            description
            copyright
            languages
            version
          }
        }
      }
    `,
    {},
  );
  return res.refresh;
};

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
