import graphql, { type Response as GraphqlResponse } from "../../graphql";

interface IItem {
  id: number;
  code: string;
  name: string;
  country: string;
  number: number;
  units?: number;
  fund?: boolean;
  createdAt: Date;
}

interface IIndexResponse {
  indexCurrency: IItem[];
}

export const index = async (): Promise<GraphqlResponse<IIndexResponse>> => {
  return graphql(
    `
      query call() {
        indexCurrency() {
          code
          message
        }
      }
    `,
    {},
  );
};
