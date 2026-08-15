import graphql from "../../graphql";

export interface IItem {
  id: number;
  code: string;
  name: string;
  country: string;
  number: number;
  units?: number;
  fund?: boolean;
}

interface IIndexResponse {
  indexCurrency: IItem[];
}

export const index = async (): Promise<IItem[]> => {
  const res: IIndexResponse = await graphql(
    `
      query call {
        indexCurrency {
          id
          code
          name
          country
          number
          units
          fund
        }
      }
    `,
    {},
  );
  return res.indexCurrency;
};
