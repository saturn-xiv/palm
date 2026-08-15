import graphql, { type Response as GraphqlResponse } from "../../graphql";

interface IIndexResponse {
  indexTimezone: string[];
}

export const index = async (): Promise<GraphqlResponse<IIndexResponse>> => {
  return graphql(
    `
      query call() {
        indexTimezone() {          
        }
      }
    `,
    {},
  );
};
