import { DeleteOutlined } from "@ant-design/icons";

import { message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { Popconfirm } from "antd";

import {
  IAttachment,
  destroy as destroy_attachment,
} from "../../../api/attachments";
import { IErrorMessage } from "../../../api/graphql";

interface IProps {
  item: IAttachment;
  handleRefresh: () => void;
}
const Widget = ({ item, handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  return (
    <Popconfirm
      title={
        <FormattedMessage
          id="personal.attachments.delete.confirm.title"
          values={{ title: item.title }}
        />
      }
      description={
        <FormattedMessage id="personal.attachments.delete.confirm.description" />
      }
      onConfirm={async () => {
        destroy_attachment(item.id)
          .then(() => {
            messageApi.info({
              type: "success",
              content: intl.formatMessage({ id: "flashes.succeed" }),
              onClose: handleRefresh,
              duration: 2,
            });
          })
          .catch((reason: IErrorMessage[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
          });
      }}
      onCancel={() => {}}
      okText={<FormattedMessage id="buttons.yes" />}
      cancelText={<FormattedMessage id="buttons.no" />}
    >
      {contextHolder}
      <DeleteOutlined />
    </Popconfirm>
  );
};

export default Widget;
