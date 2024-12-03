import type { MessageInstance } from "antd/es/message/interface";
import { PlusOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import {
  ModalForm,
  ProFormSelect,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Button, Form } from "antd";

import {
  ILedger,
  create_main_account,
  ACCOUNT_TYPES,
} from "../../../api/hyacinth";
import { IError } from "../../../api";
import {
  MEMO_MAX_LENGTH,
  MEMO_MIN_LENGTH,
  TITLE_MAX_LENGTH,
  TITLE_MIN_LENGTH,
} from "../../../components";
import { ICurrency } from "../../../api/daffodil";

interface IProps {
  messageApi: MessageInstance;
  handleRefresh: () => void;
  ledger: ILedger;
  currencies: ICurrency[];
}

interface IFormValue {
  label: string;
  memo: string;
  currency: number;
  type: string;
}

const Widget = ({ ledger, messageApi, currencies, handleRefresh }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id="pages.accounting.accounts.new.main.title" />}
      trigger={
        <Button
          icon={<PlusOutlined />}
          color="primary"
          variant="filled"
          size="small"
        >
          <FormattedMessage id="buttons.new" />
        </Button>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        const ok = await create_main_account(
          ledger.id,
          values.label,
          values.memo,
          values.type,
          values.currency
        )
          .then(async () => {
            await messageApi
              .success(intl.formatMessage({ id: "flashes.succeed" }))
              .then(() => {
                handleRefresh();
              });
            return true;
          })
          .catch((reason: IError[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
            return false;
          });
        return ok;
      }}
    >
      <ProFormText
        name="label"
        label={<FormattedMessage id="form.fields.label.label" />}
        rules={[
          { required: true },
          { min: TITLE_MIN_LENGTH, max: TITLE_MAX_LENGTH },
        ]}
      />
      <ProFormTextArea
        colProps={{ span: 24 }}
        name="memo"
        label={<FormattedMessage id="form.fields.memo.label" />}
        rules={[
          { required: true },
          { min: MEMO_MIN_LENGTH, max: MEMO_MAX_LENGTH },
        ]}
      />
      <ProFormSelect
        width="md"
        name="type"
        label={<FormattedMessage id="form.fields.type.label" />}
        options={ACCOUNT_TYPES.map((x) => {
          return {
            label: x,
            value: x,
          };
        })}
        rules={[{ required: true }]}
      />
      <ProFormSelect
        width="md"
        name="currency"
        label={<FormattedMessage id="form.fields.currency.label" />}
        options={currencies.map((x) => {
          return {
            label: `${x.code}-${x.name}`,
            value: x.id,
          };
        })}
        rules={[{ required: true }]}
      />
    </ModalForm>
  );
};

export default Widget;
