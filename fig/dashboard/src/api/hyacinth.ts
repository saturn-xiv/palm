import { IDateTimePicker, IPager, IPagination, ISucceed, query } from ".";
import {
  IAttachment,
  ICurrency,
  IPostalAddress,
  IPostalAddressFormValue,
  IPostalRecipient,
  IPostalRecipientFormValue,
} from "./daffodil";

export const ACCOUNT_TYPE_CASH = "CASH";
export const ACCOUNT_TYPE_BANK = "BANK";
export const ACCOUNT_TYPE_STOCK = "STOCK";
export const ACCOUNT_TYPE_MUTUAL_FUND = "MUTUAL_FUND";
export const ACCOUNT_TYPE_ACCOUNTS_RECEIVABLE = "ACCOUNTS_RECEIVABLE";
export const ACCOUNT_TYPE_OTHER_ASSETS = "OTHER_ASSETS";
export const ACCOUNT_TYPE_CREDIT_CARD = "CREDIT_CARD";
export const ACCOUNT_TYPE_ACCOUNTS_PAYABLE = "ACCOUNTS_PAYABLE";
export const ACCOUNT_TYPE_LIABILITY = "LIABILITY";
export const ACCOUNT_TYPE_EQUITY = "EQUITY";
export const ACCOUNT_TYPE_INCOME = "INCOME";
export const ACCOUNT_TYPE_EXPENSES = "EXPENSES";

const INDEX_ENTRY_BY_LEDGER = `
query call($id: Int!, $pager: Pager!){
    indexBookkeepingEntryByLedger(id: $id, pager: $pager){
      items{
        id, memo, amount, updatedAt, deletedAt,
        toAccount{
          id, label, memo,
          currency{id, code, name, country, units},
        },
        fromAccount{
          id, label, memo,
          currency{id, code, name, country, units},
        },
        category{id, label},
        merchant{id, label, memo},        
        transaction{id, uid, memo},
        bills{id, contentType, title, bucket, object, size},
        tradedAt{datetime, timezone}
      },
      pagination{total}
    }
}
`;
export const index_entries_by_ledger = async (
  id: number,
  pager: IPager
): Promise<IIndexEntryResponse> => {
  const res: {
    indexBookkeepingEntryByLedger: IIndexEntryResponse;
  } = await query(INDEX_ENTRY_BY_LEDGER, { id, pager });
  return res.indexBookkeepingEntryByLedger;
};

interface IIndexEntryResponse {
  items: IEntry[];
  pagination: IPagination;
}

const INDEX_ENTRY_BY_TRANSACTION = `
query call($id: Int!){
    indexBookkeepingEntryByTransaction(id: $id){      
      id, memo, updatedAt, deletedAt,
      toAccount{
        id, label, memo,
        currency{id, code, name, country, units},
      },
      fromAccount{
        id, label, memo,
        currency{id, code, name, country, units},
      },
      category{id, label},
      merchant{id, label, memo},
      bills{id, contentType, title, bucket, object, size},    
      tradedAt{datetime, timezone}      
    }
}
`;
export const index_entries_by_transaction = async (
  id: number
): Promise<IEntry[]> => {
  const res: {
    indexBookkeepingEntryByTransaction: IEntry[];
  } = await query(INDEX_ENTRY_BY_TRANSACTION, { id });
  return res.indexBookkeepingEntryByTransaction;
};
const UPDATE_ENTRY = `
mutation call($id: Int!, $form: NewBookkeeperEntryForm!){
    updateBookkeepingEntry(id: $id, form: $form){
      createdAt
    }
}
`;
export const update_entry = async (
  id: number,
  form: IEntryFormValue
): Promise<ISucceed> => {
  const res: { updateBookkeepingEntry: ISucceed } = await query(UPDATE_ENTRY, {
    id,
    form,
  });
  return res.updateBookkeepingEntry;
};
const CREATE_ENTRY = `
mutation call($transaction: Int!, $form: NewBookkeeperEntryForm!){
    createBookkeepingEntry(transaction: $transaction, form: $form){
      createdAt
    }
}
`;
export const create_entry = async (
  transaction: number,
  form: IEntryFormValue
): Promise<ISucceed> => {
  const res: { createBookkeepingEntry: ISucceed } = await query(CREATE_ENTRY, {
    transaction,
    form,
  });
  return res.createBookkeepingEntry;
};

export interface IEntry {
  id: number;
  memo: string;
  category: ICategory;
  fromAccount: IAccount;
  toAccount: IAccount;
  merchant: IMerchant;
  transaction: ITransaction;
  bills: IAttachment[];
  amount: number;
  tradedAt: IDateTimePicker;
}

export interface IEntryFormValue {
  memo: string;
  fromAccount: number;
  toAccount: number;
  category: number;
  merchant: number;
  amount: number;
  tradedAt: string;
  timezone: string;
}

const CREATE_TRANSACTION = `
mutation call($ledger: Int!, $memo: String!, $tradedAt: String!, $timezone: String!){
    createBookkeepingTransaction(ledger: $ledger, memo: $memo, tradedAt: $tradedAt, timezone: $timezone){
      createdAt
    }
}
`;
export const create_transaction = async (
  ledger: number,
  memo: string,
  tradedAt: string,
  timezone: string
): Promise<ISucceed> => {
  const res: { createBookkeepingTransaction: ISucceed } = await query(
    CREATE_TRANSACTION,
    { ledger, memo, tradedAt, timezone }
  );
  return res.createBookkeepingTransaction;
};
const UPDATE_TRANSACTION = `
mutation call($id: Int!, $memo: String!, $tradedAt: String!, $timezone: String!){
    updateBookkeepingTransaction(id: $id, memo: $memo, tradedAt: $tradedAt, timezone: $timezone){
      createdAt
    }
}
`;
export const update_transaction = async (
  id: number,
  memo: string,
  tradedAt: string,
  timezone: string
): Promise<ISucceed> => {
  const res: { updateBookkeepingTransaction: ISucceed } = await query(
    UPDATE_TRANSACTION,
    { id, memo, tradedAt, timezone }
  );
  return res.updateBookkeepingTransaction;
};
export interface ITransaction {
  id: number;
  uid: string;
  memo: string;
  tradedAt: IDateTimePicker;
  timezone: string;
  deletedAt?: Date;
  updatedAt: Date;
}

const INDEX_TRANSACTION_LEDGER = `
query call($id: Int!, $pager: Pager!){
    indexBookkeepingTransactionByLedger(id: $id, pager: $pager){
      items{
        id, uid, memo, updatedAt, deletedAt,
        tradedAt{datetime, timezone}
      },
      pagination{total}
    }
}
`;

interface IIndexTransactionResponse {
  items: ITransaction[];
  pagination: IPagination;
}
export const index_transaction_by_ledger = async (
  id: number,
  pager: IPager
): Promise<IIndexTransactionResponse> => {
  const res: {
    indexBookkeepingTransactionByLedger: IIndexTransactionResponse;
  } = await query(INDEX_TRANSACTION_LEDGER, { id, pager });
  return res.indexBookkeepingTransactionByLedger;
};

export const ACCOUNT_TYPES = [
  ACCOUNT_TYPE_CASH,
  ACCOUNT_TYPE_BANK,
  ACCOUNT_TYPE_STOCK,
  ACCOUNT_TYPE_MUTUAL_FUND,
  ACCOUNT_TYPE_ACCOUNTS_RECEIVABLE,
  ACCOUNT_TYPE_OTHER_ASSETS,

  ACCOUNT_TYPE_CREDIT_CARD,
  ACCOUNT_TYPE_ACCOUNTS_PAYABLE,

  ACCOUNT_TYPE_LIABILITY,

  ACCOUNT_TYPE_EQUITY,

  ACCOUNT_TYPE_INCOME,

  ACCOUNT_TYPE_EXPENSES,
];

export interface IAccount {
  id: number;
  label: string;
  memo: string;
  parent?: string;
  currency: ICurrency;
  type: string;
  deletedAt?: Date;
  updatedAt: Date;
}
const UPDATE_ACCOUNT = `
mutation call($id: Int!, $label: String!, $memo: String!){
    updateBookkeepingAccount(id: $id, label: $label, memo: $memo){
      createdAt
    }
}
`;
export const update_account = async (
  id: number,
  label: string,
  memo: string
): Promise<ISucceed> => {
  const res: { updateBookkeepingAccount: ISucceed } = await query(
    UPDATE_ACCOUNT,
    { id, label, memo }
  );
  return res.updateBookkeepingAccount;
};
const CREATE_MAIN_ACCOUNT = `
mutation call($ledger: Int!, $label: String!, $memo: String!, $type: BookkeeperAccountType!, $currency: Int!){
    createBookkeepingMainAccount(ledger: $ledger, label: $label, memo: $memo, type: $type, currency: $currency){
      createdAt
    }
}
`;
export const create_main_account = async (
  ledger: number,
  label: string,
  memo: string,
  type: string,
  currency: number
): Promise<ISucceed> => {
  const res: { createBookkeepingMainAccount: ISucceed } = await query(
    CREATE_MAIN_ACCOUNT,
    { ledger, label, memo, currency, type }
  );
  return res.createBookkeepingMainAccount;
};
const CREATE_SUB_ACCOUNT = `
mutation call($parent: Int!, $label: String!, $memo: String!, $type: BookkeeperAccountType!, $currency: Int!){
    createBookkeepingSubAccount(parent: $parent, label: $label, memo: $memo, type: $type, currency: $currency){
      createdAt
    }
}
`;
export const create_sub_account = async (
  parent: number,
  label: string,
  memo: string,
  type: string,
  currency: number
): Promise<ISucceed> => {
  const res: { createBookkeepingSubAccount: ISucceed } = await query(
    CREATE_SUB_ACCOUNT,
    { parent, label, memo, currency, type }
  );
  return res.createBookkeepingSubAccount;
};
const INDEX_ACCOUNT_BY_LEDGER = `
query call($id: Int!){
    indexBookkeepingAccountByLedger(id: $id){
      id, parent, label, memo, type, updatedAt, deletedAt,
      currency{id, code, name, country, units}
    }
}
`;
export const index_account_by_ledger = async (
  id: number
): Promise<IAccount[]> => {
  const res: { indexBookkeepingAccountByLedger: IAccount[] } = await query(
    INDEX_ACCOUNT_BY_LEDGER,
    { id }
  );
  return res.indexBookkeepingAccountByLedger;
};

export interface ICategory {
  id: number;
  parent?: string;
  label: string;
  deletedAt?: Date;
  updatedAt: Date;
}

const INDEX_CATEGORY_LEDGER = `
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
    INDEX_CATEGORY_LEDGER,
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

const SHARE_LEDGER = `
query call($id: Int!, $notBefore: String!, $expiresAt: String!, $timezone: String!){
    shareBookkeepingLedger(id: $id, notBefore: $notBefore, expiresAt: $expiresAt, timezone: $timezone)
}
`;

export const share_ledger = async (
  id: number,
  notBefore: string,
  expiresAt: string,
  timezone: string
): Promise<string> => {
  const res: { shareBookkeepingLedger: string } = await query(SHARE_LEDGER, {
    id,
    notBefore,
    expiresAt,
    timezone,
  });
  return res.shareBookkeepingLedger;
};
