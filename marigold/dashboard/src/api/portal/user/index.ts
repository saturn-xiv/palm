import { type ILayout as IPageLayout } from "../site";
import graphql from "../../../graphql";

export interface ISignInResponse {
  token: string;
  user: ILayout;
  page: IPageLayout;
}

export interface IRefreshResponse {
  user?: ILayout;
  page: IPageLayout;
}

export const refresh = async (lang: string): Promise<IRefreshResponse> => {
  const res: { refresh: IRefreshResponse } = await graphql(
    `
      query call($lang: String!) {
        refresh(lang: $lang) {
          user {
            lang
            timezone
            name
            avatar
            isRoot
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
          page {
            favicon
            title
            subhead
            author {
              name
              email
            }
            project {
              name
              url
              version
            }
            keywords
            description
            copyright
            locale
            languages
            buildTime
          }
        }
      }
    `,
    { lang },
  );
  return res.refresh;
};

export interface ILayout {
  lang: string;
  timezone: string;
  name: string;
  avatar: string;
  isRoot: boolean;
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
