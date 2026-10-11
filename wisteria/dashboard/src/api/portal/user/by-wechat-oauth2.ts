import graphql from "../../../graphql";
import { type ISignInResponse } from ".";

export const callback = async (
  code: string,
  state: string,
  lang: string,
  timezone: string,
): Promise<ISignInResponse> => {
  const res: { wechatOauth2UserCallback: ISignInResponse } = await graphql(
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

export const sign_in_url = async (): Promise<string> => {
  const res: { urlForWechaOauth2UserSignIn: string } = await graphql(
    `
      query call {
        urlForWechaOauth2UserSignIn
      }
    `,
    {},
  );
  return res.urlForWechaOauth2UserSignIn;
};

export const bind_url = async (): Promise<string> => {
  const res: { urlForBindWechatOauth2User: string } = await graphql(
    `
      query call {
        urlForBindWechatOauth2User
      }
    `,
    {},
  );
  return res.urlForBindWechatOauth2User;
};
