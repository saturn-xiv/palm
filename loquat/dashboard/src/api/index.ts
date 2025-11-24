import { graphql, type IGraphqlResponse } from "../request";

export interface IPage {
  size: number;
  index: number;
}

export interface IPagination {
  size: number;
  index: number;
  total: number;
}

export interface IRefreshResponse {
  createdAt: Date;
  version: string;
  hostname: string;
}

export const refresh = async (): Promise<
  IGraphqlResponse<{ refresh: IRefreshResponse }>
> => {
  const res: IGraphqlResponse<{ refresh: IRefreshResponse }> = await graphql(
    `
      {
        refresh {
          hostname
          version
        }
      }
    `,
    {}
  );
  return res;
};
