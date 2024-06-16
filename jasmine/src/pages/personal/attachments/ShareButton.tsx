import { useState } from "react";
import { ShareAltOutlined, CopyOutlined } from "@ant-design/icons";
import { ModalForm, ProForm, ProFormSelect } from "@ant-design/pro-components";
import { Form, message, Button } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { CopyToClipboard } from "react-copy-to-clipboard";

import {
  IAttachment,
  IAttachmentShowResponse,
  show_attachment_by_id,
} from "../../../api/attachments";
import { IErrorMessage } from "../../../api/graphql";

interface IProps {
  item: IAttachment;
}

interface IForm {
  days: number;
}
const Widget = ({ item }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const [form] = Form.useForm<IForm>();
  const [attachment, setAttachment] = useState<
    IAttachmentShowResponse | undefined
  >();
  return (
    <ModalForm<IForm>
      title={
        <FormattedMessage
          id="personal.attachments.share.title"
          values={{ id: item.id }}
        />
      }
      trigger={<ShareAltOutlined />}
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
        onCancel: () => {},
      }}
      submitTimeout={2000}
      onFinish={async (values) => {
        show_attachment_by_id(item.id, 60 * 60 * 24 * values.days)
          .then((res) => {
            messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            setAttachment(res);
          })
          .catch((reason: IErrorMessage[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
          });
        return false;
      }}
      initialValues={{ days: 7 }}
    >
      {contextHolder}
      <ProFormSelect
        options={Array(7)
          .fill(0)
          .map((_, i) => {
            i += 1;
            return {
              value: i,
              label: i,
            };
          })}
        width="md"
        cacheForSwr
        name="days"
        label={<FormattedMessage id="form.fields.days.label" />}
      />
      <ProForm.Group>
        {attachment && (
          <div>
            <a href={attachment.url} target="_blank" rel="noreferrer">
              {attachment.item.title}
            </a>
            &nbsp; ({attachment.item.contentType}) &nbsp;
            {attachment.item.size}kb &nbsp;
            {attachment.url && (
              <CopyToClipboard text={attachment.url}>
                <Button shape="round" icon={<CopyOutlined />} size="small" />
              </CopyToClipboard>
            )}
          </div>
        )}
      </ProForm.Group>
    </ModalForm>
  );
};

export default Widget;
