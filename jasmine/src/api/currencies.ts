import { query } from "./graphql";

export interface ICurrencyOption {
  id: number;
  code: string;
  name: string;
  unit: number;
}

export const currency_options = async (): Promise<ICurrencyOption[]> => {
  const res = await query<{
    currencyOptions: ICurrencyOption[];
  }>(
    `
  query call{
    currencyOptions{
      id, code, name, unit
    }
  }
  `,
    {}
  );
  return res.currencyOptions;
};
