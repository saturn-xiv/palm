import { EditOutlined } from "@ant-design/icons";
import {
  ModalForm,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Button, Form } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";

import {
  MEMO_MAX_LENGTH,
  MEMO_MIN_LENGTH,
  TITLE_MAX_LENGTH,
  TITLE_MIN_LENGTH,
} from "../../../components";
import { IError } from "../../../api";
import { ILedger, update_ledger } from "../../../api/hyacinth";

interface IFormValue {
  label: string;
  memo: string;
}
interface IProps {
  item: ILedger;
  messageApi: MessageInstance;
  handleReload: () => void;
}
const Widget = ({ item, messageApi, handleReload }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={item.label}
      trigger={
        <Button icon={<EditOutlined />} size="small" type="dashed">
          <FormattedMessage id="buttons.edit" />
        </Button>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        return {
          label: item.label,
          memo: item.memo,
        };
      }}
      submitter={{
        render: (_props, defaultDoms) => {
          return [
            ...defaultDoms,
            <Button
              key="disable"
              danger
              onClick={() => {
                // TODO
                console.log("disable", item.id);
              }}
            >
              <FormattedMessage id="buttons.disable" />
            </Button>,
          ];
        },
      }}
      onFinish={async (values) => {
        const ok = await update_ledger(item.id, values.label, values.memo)
          .then(async () => {
            await messageApi
              .success(intl.formatMessage({ id: "flashes.succeed" }))
              .then(() => {
                handleReload();
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
