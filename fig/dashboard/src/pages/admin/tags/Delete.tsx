import { Button, Tooltip, Popconfirm } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { DeleteOutlined } from "@ant-design/icons";
import type { MessageInstance } from "antd/es/message/interface";

import { destroy_tag, ITag } from "../../../api/daffodil";
import { IError } from "../../../api";

interface IProps {
  handleRefresh: () => void;
  messageApi: MessageInstance;
  item: ITag;
}

const Widget = ({ item, messageApi, handleRefresh }: IProps) => {
  const intl = useIntl();
  return (
    <Tooltip title={<FormattedMessage id="buttons.delete" />}>
      <Popconfirm
        title={<FormattedMessage id="flashes.are-you-sure" />}
        onConfirm={() => {
          destroy_tag(item.id)
            .then(() => {
              messageApi
                .success(
                  intl.formatMessage({
                    id: "flashes.succeed",
                  })
                )
                .then(() => {
                  handleRefresh();
                });
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        okText={<FormattedMessage id="buttons.yes" />}
        cancelText={<FormattedMessage id="buttons.no" />}
      >
        <Button icon={<DeleteOutlined />} danger shape="circle" size="small" />
      </Popconfirm>
    </Tooltip>
  );
};

export default Widget;
