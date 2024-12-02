import type { MessageInstance } from "antd/es/message/interface";
import { PlusOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import {
  ModalForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { Button, Form } from "antd";

import { ILedger, create_category, ICategory } from "../../../api/hyacinth";
import { IError } from "../../../api";
import { TITLE_MAX_LENGTH, TITLE_MIN_LENGTH } from "../../../components";

interface IProps {
  messageApi: MessageInstance;
  handleRefresh: () => void;
  ledger: ILedger;
  items: ICategory[];
}

interface IFormValue {
  label: string;
  parent?: number;
}

const Widget = ({ ledger, messageApi, items, handleRefresh }: IProps) => {
  const [form] = Form.useForm<IFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IFormValue>
      title={<FormattedMessage id="pages.accounting.categories.new.title" />}
      trigger={
        <Button icon={<PlusOutlined />} type="primary" size="small">
          <FormattedMessage id="buttons.new" />
        </Button>
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        return {
          label: "",
        };
      }}
      onFinish={async (values) => {
        const ok = await create_category(ledger.id, values.label, values.parent)
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
      {" "}
      <ProFormSelect
        width="md"
        name="parent"
        label={<FormattedMessage id="form.fields.parent.label" />}
        options={items.map((x) => {
          return {
            label: x.label,
            value: x.id,
          };
        })}
      />
      <ProFormText
        name="label"
        label={<FormattedMessage id="form.fields.label.label" />}
        rules={[
          { required: true },
          { min: TITLE_MIN_LENGTH, max: TITLE_MAX_LENGTH },
        ]}
      />
    </ModalForm>
  );
};

export default Widget;
