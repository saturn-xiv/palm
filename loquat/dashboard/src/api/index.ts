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
  refresh: { createdAt: Date; version: string; hostname: string };
}

export const refresh = async (): Promise<
  IGraphqlResponse<IRefreshResponse>
> => {
  const res: IGraphqlResponse<IRefreshResponse> = await graphql(
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
