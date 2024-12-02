import type { MessageInstance } from "antd/es/message/interface";
import { EditOutlined } from "@ant-design/icons";
import { FormattedMessage, useIntl } from "react-intl";
import { ModalForm, ProFormText } from "@ant-design/pro-components";
import { Button, Form } from "antd";

import { update_category, ICategory } from "../../../api/hyacinth";
import { IError } from "../../../api";
import { TITLE_MAX_LENGTH, TITLE_MIN_LENGTH } from "../../../components";

interface IProps {
  item: ICategory;
  messageApi: MessageInstance;
  handleRefresh: () => void;
}

interface IFormValue {
  label: string;
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
        return {
          label: item.label,
        };
      }}
      onFinish={async (values) => {
        const ok = await update_category(item.id, values.label)
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
    </ModalForm>
  );
};

export default Widget;
