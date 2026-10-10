import graphql from "../../../graphql";
import { type ISucceeded } from "..";
import { type ISignInResponse } from ".";

export const sign_in = async (
  code: string,
  lang: string,
  timezone: string,
): Promise<ISignInResponse> => {
  const res: { signInByWechatMiniProgram: ISignInResponse } = await graphql(
    `
      mutation call($code: String!, $lang: String!, $timezone: String!) {
        signInByWechatMiniProgram(
          code: $code
          lang: $lang
          timezone: $timezone
        ) {
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
    { code, lang, timezone },
  );
  return res.signInByWechatMiniProgram;
};

export const set_info = async (
  nickname: string,
  avatarUrl: string,
): Promise<ISucceeded> => {
  const res: { setWechatMiniProgramUserInfo: ISucceeded } = await graphql(
    `
      mutation call($nickname: String!, $avatarUrl: String!) {
        setWechatMiniProgramUserInfo(
          nickname: $nickname
          avatarUrl: $avatarUrl
        ) {
          createdAt
        }
      }
    `,
    { nickname, avatarUrl },
  );
  return res.setWechatMiniProgramUserInfo;
};
