import { IPager, IPagination, ISucceed, query } from ".";
import {
  IPostalAddress,
  IPostalAddressFormValue,
  IPostalRecipient,
  IPostalRecipientFormValue,
} from "./daffodil";

export interface ICategory {
  id: number;
  parent?: string;
  label: string;
  deletedAt?: Date;
  updatedAt: Date;
}

const INDEX_CATEGORY = `
query call($id: Int!){
    indexBookkeepingCategoryByLedger(id: $id){
      id, parent, label, updatedAt, deletedAt
    }
}
`;
export const index_category_by_ledger = async (
  id: number
): Promise<ICategory[]> => {
  const res: { indexBookkeepingCategoryByLedger: ICategory[] } = await query(
    INDEX_CATEGORY,
    { id }
  );
  return res.indexBookkeepingCategoryByLedger;
};

const UPDATE_CATEGORY = `
mutation call($id: Int!, $label: String!){
    updateBookkeepingCategory(id: $id, label: $label){
      createdAt
    }
}
`;
export const update_category = async (
  id: number,
  label: string
): Promise<ISucceed> => {
  const res: { updateBookkeepingCategory: ISucceed } = await query(
    UPDATE_CATEGORY,
    { id, label }
  );
  return res.updateBookkeepingCategory;
};
const CREATE_CATEGORY = `
mutation call($ledger: Int!, $parent: Int, $label: String!){
    createBookkeepingCategory(ledger: $ledger, parent: $parent, label: $label){
      createdAt
    }
}
`;
export const create_category = async (
  ledger: number,
  label: string,
  parent?: number
): Promise<ISucceed> => {
  const res: { createBookkeepingCategory: ISucceed } = await query(
    CREATE_CATEGORY,
    { ledger, parent, label }
  );
  return res.createBookkeepingCategory;
};

const SET_MERCHANT_CONTACT = `
mutation call($id: Int!, $form: PostalRecipientForm!){
    setBookkeepingMerchantContact(id: $id, form: $form){
      createdAt
    }
}
`;

export const set_merchant_contact = async (
  id: number,
  form: IPostalRecipientFormValue
): Promise<ISucceed> => {
  const res: { setBookkeepingMerchantContact: ISucceed } = await query(
    SET_MERCHANT_CONTACT,
    { id, form }
  );
  return res.setBookkeepingMerchantContact;
};
const SET_MERCHANT_ADDRESS = `
mutation call($id: Int!, $form: PostalAddressForm!){
    setBookkeepingMerchantAddress(id: $id, form: $form){
      createdAt
    }
}
`;

export const set_merchant_address = async (
  id: number,
  form: IPostalAddressFormValue
): Promise<ISucceed> => {
  const res: { setBookkeepingMerchantAddress: ISucceed } = await query(
    SET_MERCHANT_ADDRESS,
    { id, form }
  );
  return res.setBookkeepingMerchantAddress;
};

const CREATE_MERCHANT = `
mutation call($ledger: Int!, $label: String!, $memo: String!){
    createBookkeepingMerchant(ledger: $ledger, label: $label, memo: $memo){
      createdAt
    }
}
`;

export const create_merchant = async (
  ledger: number,
  label: string,
  memo: string
): Promise<ISucceed> => {
  const res: { createBookkeepingMerchant: ISucceed } = await query(
    CREATE_MERCHANT,
    { ledger, label, memo }
  );
  return res.createBookkeepingMerchant;
};

const UPDATE_MERCHANT = `
mutation call($id: Int!, $label: String!, $memo: String!){
    updateBookkeepingMerchant(id: $id, label: $label, memo: $memo){
      createdAt
    }
}
`;

export const update_merchant = async (
  id: number,
  label: string,
  memo: string
): Promise<ISucceed> => {
  const res: { updateBookkeepingMerchant: ISucceed } = await query(
    UPDATE_MERCHANT,
    { id, label, memo }
  );
  return res.updateBookkeepingMerchant;
};

const INDEX_MERCHANT_BY_LEDGER = `
query call($id: Int!){
    indexBookkeepingMerchantByLedger(id: $id){
      id, label, memo, deletedAt, updatedAt,
      contact{id, name, email, fax, phone, whatsapp, wechat, updatedAt, deletedAt},
      address{id, unit, building, street, city, province, country, passcode, zipCode, updatedAt, deletedAt}
    }
}
`;
export interface IMerchant {
  id: number;
  label: string;
  memo: string;
  contact?: IPostalRecipient;
  address?: IPostalAddress;
  deletedAt?: Date;
  updatedAt: Date;
}

export const index_merchant_by_ledger = async (
  id: number
): Promise<IMerchant[]> => {
  const res: { indexBookkeepingMerchantByLedger: IMerchant[] } = await query(
    INDEX_MERCHANT_BY_LEDGER,
    { id }
  );
  return res.indexBookkeepingMerchantByLedger;
};

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
