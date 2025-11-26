import type { IOk } from ".";
import { graphql, type IGraphqlResponse } from "../request";

interface IRebootResponse {
  reboot: IOk;
}
export const reboot = async (): Promise<IGraphqlResponse<IRebootResponse>> => {
  const res: IGraphqlResponse<IRebootResponse> = await graphql(
    `
      mutation call {
        reboot {
          createdAt
        }
      }
    `,
    {}
  );
  return res;
};

interface IApplyResponse {
  apply: IOk;
}
export const apply = async (
  run: boolean
): Promise<IGraphqlResponse<IApplyResponse>> => {
  const res: IGraphqlResponse<IApplyResponse> = await graphql(
    `
      mutation call($run: Boolean!) {
        apply(run: $run) {
          createdAt
        }
      }
    `,
    { run }
  );
  return res;
};
