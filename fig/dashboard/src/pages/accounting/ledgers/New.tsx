import { PlusOutlined } from "@ant-design/icons";
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
import { create_ledger } from "../../../api/hyacinth";

interface IFormValue {
  label: string;
  memo: string;
}
interface IProps {
  messageApi: MessageInstance;
  handleReload: () => void;
}
const Widget = ({ messageApi, handleReload }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id="pages.accounting.ledgers.new.title" />}
      trigger={
        <Button icon={<PlusOutlined />} type="primary">
          <FormattedMessage id="buttons.new" />
        </Button>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      onFinish={async (values) => {
        const ok = await create_ledger(values.label, values.memo)
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
