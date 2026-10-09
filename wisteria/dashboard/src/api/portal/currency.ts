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

export const index = async (): Promise<IItem[]> => {
  const res: { indexCurrency: IItem[] } = await graphql(
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
