import { graphql, type IGraphqlResponse } from "../request";
import type { IPage, IPagination } from ".";

export interface ILog {
  id: string;
  ip: string;
  message: string;
  createdAt: Date;
}

export interface IIndexLogResponse {
  indexLog: { items: ILog[]; pagination: IPagination };
}

export const index_log = async (
  page: IPage
): Promise<IGraphqlResponse<IIndexLogResponse>> => {
  const res: IGraphqlResponse<IIndexLogResponse> = await graphql(
    `
      call($page: Page!) {
        indexLog(page: $page) {
          items {id, ip, message, createdAt}
          pagination {index, size, total, hasPrevious, hasNext}
        }
      }
    `,
    { page }
  );
  return res;
};

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
