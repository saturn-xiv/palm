import graphql from "../../../graphql";
import { type ISucceeded } from "..";
import { type ISignInResponse } from ".";

export const set_password = async (
  id: number,
  password: string,
): Promise<ISucceeded> => {
  const res: { setPasswordForEmailUser: ISucceeded } = await graphql(
    `
      mutation call($id: Int!, $password: String!) {
        setPasswordForEmailUser(id: $id, password: $password) {
          createdAt
        }
      }
    `,
    { id, password },
  );
  return res.setPasswordForEmailUser;
};

export const sign_in = async (): Promise<ISignInResponse> => {
  const res: { signInByEmail: ISignInResponse } = await graphql(
    `
      mutation call($email: String!, $password: String!) {
        signInByEmail(email: $email, password: $password) {
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
    {},
  );
  return res.signInByEmail;
};

export const reset_password = async (
  token: string,
  password: string,
): Promise<ISucceeded> => {
  const res: { resetPasswordForEmailUser: ISucceeded } = await graphql(
    `
      mutation call($token: String!, $password: String!) {
        resetPasswordForEmailUser(token: $token, password: $password) {
          createdAt
        }
      }
    `,
    { token, password },
  );
  return res.resetPasswordForEmailUser;
};
export const forgot_password = async (email: string): Promise<ISucceeded> => {
  const res: { forgotPasswordForEmailUser: ISucceeded } = await graphql(
    `
      query call($email: String!) {
        indexCurrency(email: $email) {
          createdAt
        }
      }
    `,
    { email },
  );
  return res.forgotPasswordForEmailUser;
};

export const unlock = async (email: string): Promise<ISucceeded> => {
  const res: { unlockForEmailUser: ISucceeded } = await graphql(
    `
      query call($email: String!) {
        unlockForEmailUser(email: $email) {
          createdAt
        }
      }
    `,
    { email },
  );
  return res.unlockForEmailUser;
};

export const unlock_by_token = async (token: string): Promise<ISucceeded> => {
  const res: { unlockForEmailUser: ISucceeded } = await graphql(
    `
      mutation call($token: String!) {
        unlockForEmailUser(token: $token) {
          createdAt
        }
      }
    `,
    { token },
  );
  return res.unlockForEmailUser;
};

export const confirm = async (email: string): Promise<ISucceeded> => {
  const res: { confirmForEmailUser: ISucceeded } = await graphql(
    `
      query call($email: String!) {
        confirmForEmailUser(email: $email) {
          createdAt
        }
      }
    `,
    { email },
  );
  return res.confirmForEmailUser;
};

export const confirm_by_token = async (token: string): Promise<ISucceeded> => {
  const res: { confirmForEmailUser: ISucceeded } = await graphql(
    `
      mutation call($token: String!) {
        confirmForEmailUser(token: $token) {
          createdAt
        }
      }
    `,
    { token },
  );
  return res.confirmForEmailUser;
};

export const sign_up = async (
  name: string,
  email: string,
  password: string,
  lang: string,
  timezone: string,
): Promise<ISucceeded> => {
  const res: { signUpByEmail: ISucceeded } = await graphql(
    `
      mutation call($request: UserSignUpByEmailRequest!) {
        signUpByEmail(request: $request) {
          createdAt
        }
      }
    `,
    { name, email, password, lang, timezone },
  );
  return res.signUpByEmail;
};
