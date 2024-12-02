import type { MessageInstance } from "antd/es/message/interface";
import { PlusOutlined, EditOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import {
  ModalForm,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Button, Form } from "antd";

import {
  create_merchant,
  ILedger,
  IMerchant,
  update_merchant,
} from "../../../api/hyacinth";
import { IError } from "../../../api";
import {
  MEMO_MAX_LENGTH,
  MEMO_MIN_LENGTH,
  TITLE_MAX_LENGTH,
  TITLE_MIN_LENGTH,
} from "../../../components";

interface IProps {
  item?: IMerchant;
  messageApi: MessageInstance;
  handleRefresh: () => void;
  ledger: ILedger;
}

interface IFormValue {
  label: string;
  memo: string;
}

const Widget = ({ ledger, messageApi, item, handleRefresh }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={
        item ? (
          item.label
        ) : (
          <FormattedMessage id="pages.accounting.merchants.new.title" />
        )
      }
      trigger={
        item ? (
          <Button icon={<EditOutlined />} variant="dashed" size="small">
            <FormattedMessage id="buttons.edit" />
          </Button>
        ) : (
          <Button icon={<PlusOutlined />} type="primary" size="small">
            <FormattedMessage id="buttons.new" />
          </Button>
        )
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        return {
          label: item?.label || "",
          memo: item?.memo || "",
        };
      }}
      onFinish={async (values) => {
        if (item) {
          const ok = await update_merchant(item.id, values.label, values.memo)
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
        }

        const ok = await create_merchant(ledger.id, values.label, values.memo)
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
    </ModalForm>
  );
};

export default Widget;
