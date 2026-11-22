import graphql from "../../graphql";

interface IIndexResponse {
  indexTimezone: string[];
}

export const index = async (): Promise<string[]> => {
  const res: IIndexResponse = await graphql(
    `
      query call {
        indexTimezone
      }
    `,
    {},
  );
  return res.indexTimezone;
};
