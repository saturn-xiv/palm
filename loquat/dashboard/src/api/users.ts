import { graphql, type IGraphqlResponse } from "../request";

export interface ISignInFormValues {
  name: string;
  password: string;
}
interface ISignInResponse {
  signIn: { username: string; token: string };
}

export const sign_in = async (
  account: ISignInFormValues
): Promise<IGraphqlResponse<ISignInResponse>> => {
  const res: IGraphqlResponse<ISignInResponse> = await graphql(
    `
      mutation call($account: Account!, $ttl: Int!) {
        signIn(account: $account, ttl: $ttl) {
          username
          token
        }
      }
    `,
    { account, ttl: 60 * 60 * 24 }
  );
  return res;
};
