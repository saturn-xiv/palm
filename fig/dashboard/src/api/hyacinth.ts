import { IPager, IPagination, ISucceed, query } from ".";

const INDEX_LOG_BY_LEDGER = `
query call($id: Int!, $pager: Pager!){
    indexBookkeepingLogByLedger(id: $id, pager: $pager){
      items{id, ledgerId, userId, username, action, memo, reason, ip, createdAt},
      pagination{total}
    }
}
`;
export interface ILog {
  id: number;
  ledgerId: number;
  userId: number;
  username: string;
  action: string;
  memo: string;
  reason?: string;
  ip: string;
  createdAt: Date;
}
interface IndexLogResponse {
  pagination: IPagination;
  items: ILog[];
}
export const index_log_by_ledger = async (
  id: number,
  pager: IPager
): Promise<IndexLogResponse> => {
  const res: { indexBookkeepingLogByLedger: IndexLogResponse } = await query(
    INDEX_LOG_BY_LEDGER,
    { id, pager }
  );
  return res.indexBookkeepingLogByLedger;
};

const SHOW_LEDGER = `
query call($id: Int!){
    showBookkeepingLedger(id: $id){
      id, uid, label, memo, deletedAt, updatedAt
    }
}
`;

export const show_ledger = async (id: number): Promise<ILedger> => {
  const res: { showBookkeepingLedger: ILedger } = await query(SHOW_LEDGER, {
    id,
  });
  return res.showBookkeepingLedger;
};

const UPDATE_LEDGER = `
mutation call($id: Int!, $label: String!, $memo: String!){
    updateBookkeepingLedger(id: $id, label: $label, memo: $memo){
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
