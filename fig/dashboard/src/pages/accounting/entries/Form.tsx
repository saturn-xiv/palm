import { BranchesOutlined, EditOutlined } from "@ant-design/icons";
import { FormattedMessage } from "react-intl";
import {
  ModalForm,
  ProForm,
  ProFormDateTimePicker,
  ProFormMoney,
  ProFormSelect,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Button, Form, Tooltip } from "antd";
import dayjs from "dayjs";
import { useEffect, useState } from "react";

import {
  IAccount,
  ICategory,
  IEntry,
  IEntryFormValue,
  ILedger,
  IMerchant,
  index_account_by_ledger,
  index_category_by_ledger,
  index_merchant_by_ledger,
} from "../../../api/hyacinth";
import {
  DATETIME_ISO_FORMAT,
  from_cents,
  MEMO_MAX_LENGTH,
  MEMO_MIN_LENGTH,
  to_cents,
} from "../../../components";
import { ICurrency } from "../../../api/daffodil";
import { guess_timezone, timezones } from "../../../utils";

interface IProps {
  ledger: ILedger;
  handleSave: (values: IEntryFormValue) => Promise<boolean>;
  title: string;
  currencies: ICurrency[];
  item?: IEntry;
}

interface IFormValue extends IEntryFormValue {
  currency: string;
}

const Widget = ({ ledger, currencies, title, handleSave, item }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const [accounts, setAccounts] = useState<IAccount[]>([]);
  const [categories, setCategories] = useState<ICategory[]>([]);
  const [merchants, setMerchants] = useState<IMerchant[]>([]);
  const [currency, setCurrency] = useState<ICurrency | undefined>(
    item?.fromAccount.currency
  );

  useEffect(() => {
    index_account_by_ledger(ledger.id).then(setAccounts);
    index_category_by_ledger(ledger.id).then(setCategories);
    index_merchant_by_ledger(ledger.id).then(setMerchants);
  }, [ledger]);
  return currencies.length > 0 &&
    accounts.length > 1 &&
    categories.length > 0 &&
    merchants.length > 0 ? (
    <ModalForm<IFormValue>
      title={title}
      trigger={
        <Tooltip title={title}>
          <Button
            icon={item ? <EditOutlined /> : <BranchesOutlined />}
            variant="dashed"
            size="small"
          >
            <FormattedMessage id={`buttons.${item ? "edit" : "new"}`} />
          </Button>
        </Tooltip>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        if (currency) {
          const ok = await handleSave({
            memo: values.memo,
            toAccount: values.toAccount,
            fromAccount: values.fromAccount,
            category: values.category,
            merchant: values.merchant,
            amount: to_cents(currency, values.amount),
            tradedAt: values.tradedAt,
            timezone: values.timezone,
          });
          return ok;
        }
      }}
      request={async () => {
        return {
          memo: item?.memo || "",
          toAccount: item?.toAccount.id || accounts[0].id,
          fromAccount: item?.fromAccount.id || accounts[1].id,
          category: item?.category.id || categories[0].id,
          amount: item && currency ? from_cents(currency, item.amount) : 0.0,
          merchant: item?.merchant.id || merchants[0].id,
          currency: item
            ? `${item.fromAccount.currency.code}-${item.fromAccount.currency.country}`
            : "",
          tradedAt:
            item?.tradedAt.datetime ||
            dayjs(new Date()).format(DATETIME_ISO_FORMAT),
          timezone: item?.tradedAt.timezone || guess_timezone(),
        };
      }}
    >
      <ProFormTextArea
        colProps={{ span: 24 }}
        name="memo"
        label={<FormattedMessage id="form.fields.memo.label" />}
        rules={[
          { required: true },
          { min: MEMO_MIN_LENGTH, max: MEMO_MAX_LENGTH },
        ]}
      />
      <ProForm.Group>
        <ProFormSelect
          width="md"
          name="fromAccount"
          label={
            <FormattedMessage id="pages.accounting.entries.form.fields.from-account.label" />
          }
          onChange={(it) => {
            for (const ia of accounts) {
              if (ia.id === it) {
                form.setFieldValue(
                  "currency",
                  `${ia.currency.code}-${ia.currency.country}`
                );
                setCurrency(ia.currency);
                return;
              }
            }
          }}
          options={accounts.map((x) => {
            return {
              label: x.label,
              value: x.id,
            };
          })}
          rules={[{ required: true }]}
        />
        <ProFormSelect
          width="md"
          name="toAccount"
          label={
            <FormattedMessage id="pages.accounting.entries.form.fields.to-account.label" />
          }
          options={accounts
            .filter(
              (x) =>
                x.currency.id === currency?.id &&
                x.id !== form.getFieldValue("fromAccount")
            )
            .map((x) => {
              return {
                label: x.label,
                value: x.id,
              };
            })}
          rules={[{ required: true }]}
        />
      </ProForm.Group>
      <ProFormSelect
        width="md"
        name="merchant"
        label={<FormattedMessage id="form.fields.merchant.label" />}
        options={merchants.map((x) => {
          return {
            label: x.label,
            value: x.id,
          };
        })}
        rules={[{ required: true }]}
      />
      <ProFormSelect
        width="md"
        name="category"
        label={<FormattedMessage id="form.fields.category.label" />}
        options={categories.map((x) => {
          return {
            label: x.label,
            value: x.id,
          };
        })}
        rules={[{ required: true }]}
      />
      <ProForm.Group>
        <ProFormText
          width="md"
          name="currency"
          label={<FormattedMessage id="form.fields.currency.label" />}
          disabled
          rules={[{ required: true }]}
        />
        <ProFormMoney
          width="md"
          name="amount"
          label={<FormattedMessage id="form.fields.amount.label" />}
          fieldProps={{
            numberPopoverRender: true,
          }}
        />
      </ProForm.Group>
      <ProForm.Group>
        <ProFormSelect
          width="md"
          name="timezone"
          label={<FormattedMessage id="form.fields.timezone.label" />}
          options={timezones().map((x) => {
            return {
              label: x,
              value: x,
            };
          })}
          rules={[{ required: true }]}
        />
        <ProFormDateTimePicker
          name="tradedAt"
          label={<FormattedMessage id="form.fields.traded-at.label" />}
          rules={[{ required: true }]}
        />
      </ProForm.Group>
    </ModalForm>
  ) : (
    <></>
  );
};

export default Widget;
