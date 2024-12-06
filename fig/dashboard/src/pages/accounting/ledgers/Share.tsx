import type { MessageInstance } from "antd/es/message/interface";
import { ShareAltOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import { useState } from "react";
import {
  ModalForm,
  ProForm,
  ProFormDateTimePicker,
  ProFormSelect,
} from "@ant-design/pro-components";
import { Button, Form, Tooltip, Typography } from "antd";
import dayjs from "dayjs";

import { ILedger, share_ledger } from "../../../api/hyacinth";
import { IError } from "../../../api";
import { guess_timezone, home_url, timezones } from "../../../utils";
import { DATETIME_ISO_FORMAT } from "../../../components";

interface IProps {
  item: ILedger;
  messageApi: MessageInstance;
}

interface IFormValue {
  expiresAt: string;
  notBefore: string;
  timezone: string;
}

const Widget = ({ messageApi, item }: IProps) => {
  const [url, setUrl] = useState<string>();
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={item.label}
      trigger={
        <Tooltip title={<FormattedMessage id="buttons.share" />}>
          <Button icon={<ShareAltOutlined />} size="small" />
        </Tooltip>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        const nbf = dayjs();
        const exp = nbf.add(7, "day");
        return {
          notBefore: nbf.format(DATETIME_ISO_FORMAT),
          expiresAt: exp.format(DATETIME_ISO_FORMAT),
          timezone: guess_timezone(),
        };
      }}
      onFinish={async (values) => {
        await share_ledger(
          item.id,
          values.notBefore,
          values.expiresAt,
          values.timezone
        )
          .then(async (res) => {
            setUrl(`${home_url()}accounting/ledgers/${res}/`);
            await messageApi.success(
              intl.formatMessage({ id: "flashes.succeed" })
            );
          })
          .catch((reason: IError[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
          });
        return false;
      }}
    >
      <ProForm.Group>
        {url && <Typography.Paragraph copyable>{url}</Typography.Paragraph>}
      </ProForm.Group>
      <ProFormDateTimePicker
        name="notBefore"
        label={<FormattedMessage id="form.fields.not-before.label" />}
        rules={[{ required: true }]}
      />
      <ProFormDateTimePicker
        name="expiresAt"
        label={<FormattedMessage id="form.fields.expires-at.label" />}
        rules={[{ required: true }]}
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
    </ModalForm>
  );
};

export default Widget;
