import type { MessageInstance } from "antd/es/message/interface";
import { EditOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import {
  ModalForm,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Button, Form } from "antd";

import { IAccount, update_account } from "../../../api/hyacinth";
import { IError } from "../../../api";
import {
  MEMO_MAX_LENGTH,
  MEMO_MIN_LENGTH,
  TITLE_MAX_LENGTH,
  TITLE_MIN_LENGTH,
} from "../../../components";

interface IProps {
  messageApi: MessageInstance;
  handleRefresh: () => void;
  item: IAccount;
}

interface IFormValue {
  label: string;
  memo: string;
}

const Widget = ({ messageApi, item, handleRefresh }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={item.label}
      trigger={
        <Button icon={<EditOutlined />} variant="dashed" size="small">
          <FormattedMessage id="buttons.edit" />
        </Button>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        return { label: item.label, memo: item.memo };
      }}
      onFinish={async (values) => {
        const ok = await update_account(item.id, values.label, values.memo)
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
