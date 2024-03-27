import { query } from "./graphql";
import {
  ICurrencyOption,
  IAttachmentShow,
  IUserDetails,
  ISucceed,
  IAttachment,
} from "./camelia";

export const destroy_bill = async (
  id: number,
  reason: string
): Promise<ISucceed> => {
  const res = await query<{ daffodilDestroyBill: ISucceed }>(
    `
  mutation call($id: Int!, $reason: String!){
    daffodilDestroyBill(id: $id, reason: $reason){
      createdAt
    }
  }
  `,
    { id, reason }
  );
  return res.daffodilDestroyBill;
};

export interface IBill {
  id: number;
  ledger: number;
  user: IUserDetails;
  summary: string;
  amount: number;
  currency: ICurrencyOption;
  merchant: string;
  category: string;
  paidAt: Date;
  paidBy: string;
  attachments: IAttachment[];
  updatedAt: Date;
}

export const show_bill = async (id: number): Promise<IBill> => {
  const res = await query<{
    daffodilShowBill: IBill;
  }>(
    `
query call($id: Int!){
  daffodilShowBill(id: $id){    
    id, ledger,
    user{nickname, realName, avatar},
    summary, amount, 
    currency{id, code, name, unit},
    merchant, category, paidAt, paidBy, 
    attachments{id, title, bucket, name, size, contentType, status, updatedAt},
    updatedAt,    
  },
}
`,
    { id }
  );
  return res.daffodilShowBill;
};

export const bills_by_ledger = async (ledger: number): Promise<IBill[]> => {
  const res = await query<{
    daffodilIndexBill: { items: IBill[] };
  }>(
    `
query call($ledger: Int!){
  daffodilIndexBill(ledger: $ledger){
    items{
      id, ledger,
      user{nickname, realName, avatar},
      summary, amount, 
      currency{id, code, name, unit},
      merchant, category, paidAt, paidBy, 
      attachments{id, title, bucket, name, size, contentType, status, updatedAt},
      updatedAt,
    }
  },
}
`,
    { ledger }
  );
  return res.daffodilIndexBill.items;
};

export const bill_form_options = async (): Promise<{
  currencies: ICurrencyOption[];
  categories: string[];
  payment_methods: string[];
  merchants: string[];
}> => {
  const res = await query<{
    currencyOptions: ICurrencyOption[];
    daffodilBillCategories: string[];
    daffodilBillMerchants: string[];
    daffodilBillPaymentMethods: string[];
  }>(
    `
query call{
  currencyOptions{
    id, code, name, unit
  },
  daffodilBillCategories,
  daffodilBillMerchants,
  daffodilBillPaymentMethods,
}
`,
    {}
  );
  return {
    currencies: res.currencyOptions,
    categories: res.daffodilBillCategories,
    merchants: res.daffodilBillMerchants,
    payment_methods: res.daffodilBillPaymentMethods,
  };
};

export const create_bill = async (
  ledger: number,
  summary: string,
  amount: number,
  currency: string,
  merchant: string,
  category: string,
  paidAt: string,
  paidBy: string
): Promise<ISucceed> => {
  const res = await query<{ daffodilCreateBill: ISucceed }>(
    `
  mutation call($ledger: Int!, $summary: String!, $amount: Int!, $currency: String!, $merchant: String!, $category: String!, $paidAt: LocalDateTime!, $paidBy: String!){
    daffodilCreateBill(
      ledger: $ledger, 
      form: {summary: $summary, amount: $amount, currency: $currency, merchant: $merchant, category: $category, paidAt: $paidAt, paidBy: $paidBy}
    ){
      createdAt
    }
  }
  `,
    { ledger, summary, amount, currency, merchant, category, paidAt, paidBy }
  );
  return res.daffodilCreateBill;
};
export const update_ledger = async (
  id: number,
  name: string,
  summary: string,
  cover: number
): Promise<ISucceed> => {
  const res = await query<{ daffodilUpdateLedger: ISucceed }>(
    `
  mutation call($id: Int!, $name: String!, $summary: String!, $cover: Int!){
    daffodilUpdateLedger(id: $id, name: $name, summary: $summary, cover: $cover){
      createdAt
    }
  }
  `,
    { id, name, summary, cover }
  );
  return res.daffodilUpdateLedger;
};
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
  cover: IAttachmentShow;
  updatedAt: Date;
}

export const export_ledger = async (
  id: number,
  format: string
): Promise<ISucceed> => {
  const res = await query<{ daffodilExportLedger: ISucceed }>(
    `
  query call($id: Int!, $format: DaffodilLedgerExportFormat!){
    daffodilExportLedger(id: $id, format: $format){ createdAt }
  }
  `,
    { id, format }
  );
  return res.daffodilExportLedger;
};
export const share_ledger = async (
  id: number,
  begin: string,
  end: string
): Promise<string> => {
  const res = await query<{ daffodilShareLedger: string }>(
    `
  query call($id: Int!, $begin: String!, $end: String!){
    daffodilShareLedger(id: $id, begin: $begin, end: $end)
  }
  `,
    { id, begin, end }
  );
  return res.daffodilShareLedger;
};
export const show_ledger = async (id: number): Promise<ILedger> => {
  const res = await query<{ daffodilShowLedger: ILedger }>(
    `
  query call($id: Int!){
    daffodilShowLedger(id: $id){      
      id, name, 
      cover{id, title, url},
      summary, updatedAt      
    }
  }
  `,
    { id }
  );
  return res.daffodilShowLedger;
};
export const index_ledger = async (): Promise<ILedger[]> => {
  const res = await query<{ daffodilIndexLedger: { items: ILedger[] } }>(
    `
  query call{
    daffodilIndexLedger{
      items{
        id, name, 
        cover{id, title, url}, 
        summary, updatedAt
      }
    }
  }
  `,
    {}
  );
  return res.daffodilIndexLedger.items;
};
