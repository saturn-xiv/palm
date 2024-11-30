import { ISucceed, query } from ".";

const UPDATE_LEDGER = `
mutation call($id: Int!, $label: String!, $memo: String!){
    updateBookkeepingLedger(id:$id, label: $label, memo: $memo){
      createdAt
    }
}
`;

export const update_ledger = async (
  id: number,
  label: string,
  memo: string
): Promise<ISucceed> => {
  const res: { updateBookkeepingLedger: ISucceed } = await query(
    UPDATE_LEDGER,
    { id, label, memo }
  );
  return res.updateBookkeepingLedger;
};
const CREATE_LEDGER = `
mutation call($label: String!, $memo: String!){
    createBookkeepingLedger(label: $label, memo: $memo){
      createdAt
    }
}
`;

export const create_ledger = async (
  label: string,
  memo: string
): Promise<ISucceed> => {
  const res: { createBookkeepingLedger: ISucceed } = await query(
    CREATE_LEDGER,
    {
      label,
      memo,
    }
  );
  return res.createBookkeepingLedger;
};

const INDEX_LEDGER = `
query call{
    indexBookkeepingLedger{
      id, uid, label, memo, deletedAt, updatedAt
    }
}
`;
export interface ILedger {
  id: number;
  uid: string;
  label: string;
  memo: string;
  deletedAt?: Date;
  updatedAt: Date;
}

export const index_ledger = async (): Promise<ILedger[]> => {
  const res: { indexBookkeepingLedger: ILedger[] } = await query(
    INDEX_LEDGER,
    {}
  );
  return res.indexBookkeepingLedger;
};
