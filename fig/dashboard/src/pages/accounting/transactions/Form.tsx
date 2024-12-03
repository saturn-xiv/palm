import type { MessageInstance } from "antd/es/message/interface";
import { PlusOutlined, EditOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import {
  ModalForm,
  ProFormDateTimePicker,
  ProFormSelect,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Button, Form, Typography } from "antd";
import dayjs from "dayjs";

import {
  create_transaction,
  ILedger,
  ITransaction,
  update_transaction,
} from "../../../api/hyacinth";
import { IError } from "../../../api";
import {
  DATETIME_ISO_FORMAT,
  MEMO_MAX_LENGTH,
  MEMO_MIN_LENGTH,
} from "../../../components";
import { guess_timezone, timezones } from "../../../utils";

interface IProps {
  item?: ITransaction;
  messageApi: MessageInstance;
  handleRefresh: () => void;
  ledger: ILedger;
}

interface IFormValue {
  memo: string;
  tradedAt: string;
  timezone: string;
}

const Widget = ({ ledger, messageApi, item, handleRefresh }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={
        item ? (
          <Typography.Paragraph
            ellipsis={{
              rows: 1,
              expandable: false,
              expanded: false,
            }}
          >
            {item.memo}
          </Typography.Paragraph>
        ) : (
          <FormattedMessage id="pages.accounting.transactions.new.title" />
        )
      }
      trigger={
        item ? (
          <Button icon={<EditOutlined />} variant="dashed" size="small">
            <FormattedMessage id="buttons.edit" />
          </Button>
        ) : (
          <Button
            icon={<PlusOutlined />}
            color="primary"
            variant="filled"
            size="small"
          >
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
          memo: item?.memo || "",
          tradedAt:
            item?.tradedAt.datetime ||
            dayjs(new Date()).format(DATETIME_ISO_FORMAT),
          timezone: item?.tradedAt.timezone || guess_timezone(),
        };
      }}
      onFinish={async (values) => {
        if (item) {
          const ok = await update_transaction(
            item.id,
            values.memo,
            values.tradedAt,
            values.timezone
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
        }

        const ok = await create_transaction(
          ledger.id,
          values.memo,
          values.tradedAt,
          values.timezone
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
    </ModalForm>
  );
};

export default Widget;
