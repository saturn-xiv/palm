import { query } from "./graphql";
import { ISucceed } from "./camelia";

export const create_ledger = async (
  name: string,
  summary: string,
  cover: number
): Promise<ISucceed> => {
  const res = await query<{ daffodilCreateLedger: ISucceed }>(
    `
  mutation call($name: String!, $summary: String!, $cover: Int!){
    daffodilCreateLedger(name: $name, summary: $summary, cover: $cover){
      createdAt
    }
  }
  `,
    { name, summary, cover }
  );
  return res.daffodilCreateLedger;
};

export interface ILedger {
  id: number;
  name: string;
  summary: string;
  cover: string | null;
  updatedAt: number;
}

export const index_ledger = async (): Promise<ILedger[]> => {
  const res = await query<{ daffodilIndexLedger: { items: ILedger[] } }>(
    `
  query call{
    daffodilIndexLedger{
      items{
        id, name, cover, summary, updatedAt
      }
    }
  }
  `,
    {}
  );
  return res.daffodilIndexLedger.items;
};
