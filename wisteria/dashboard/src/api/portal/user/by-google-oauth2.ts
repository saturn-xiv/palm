import graphql from "../../../graphql";
import { type ISucceeded } from "..";
import { type ISignInResponse } from ".";


export const callback = async (
  code: string,
  state: string,
  lang: string,
  timezone: string,
): Promise<ISignInResponse> => {
  const res: { googleOauth2UserCallback: ISignInResponse } = await graphql(
    `
      mutation call($form: WechatOauth2UserCallbackRequest) {
        wechatOauth2UserCallback(form: $form) {
          token
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
    { form: { code, state, lang, timezone } },
  );
  return res.wechatOauth2UserCallback;
};

export const url_for_sign_in = async (redirectUri: string): Promise<string> => {
  const res: { urlForGoogleOauth2UserSignIn: string } = await graphql(
    `
      query call($redirectUri: String!) {
        urlForWechaOauth2UserSignIn(redirectUri: $redirectUri)
      }
    `,
    {redirectUri},
  );
  return res.urlForGoogleOauth2UserSignIn;
};

export const url_for_bind = async (redirectUri: string): Promise<string> => {
  const res: { urlForBindGoogleOauth2User: string } = await graphql(
    `
      query call($redirectUri: String!) {
        urlForBindGoogleOauth2User(redirectUri: $redirectUri)
      }
    `,
    {redirectUri},
  );
  return res.urlForBindGoogleOauth2User;
};


export const bind_email = async (email: string): Promise<ISucceeded> => {
  const res: { bindEmailForGoogleOauth2User: ISucceeded } = await graphql(
    `
      query call($email: String!) {
        bindEmailForGoogleOauth2User(email: $email) {
          createdAt
        }
      }
    `,
    { email },
  );
  return res.bindEmailForGoogleOauth2User;
};
