import graphql from "../../../graphql";
import { type ISignInResponse } from ".";

export const sign_in = async (
  code: string,
  state: string,
  lang: string,
  timezone: string,
): Promise<ISignInResponse> => {
  const res: { signInByWechatMiniProgram: ISignInResponse } = await graphql(
    `
      mutation call($form: UserSignInByWechatOauth2Request) {
        signInByWechatMiniProgram(form: $form) {
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
  return res.signInByWechatMiniProgram;
};

export const sign_in_url = async (): Promise<string> => {
  const res: { signInUrlForWechaOauth2: string } = await graphql(
    `
      mutation call {
        signInUrlForWechaOauth2
      }
    `,
    {},
  );
  return res.signInUrlForWechaOauth2;
};
