import { useState } from "react";
import { Button, Modal, Typography, Tooltip, Space, Popconfirm } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { EyeOutlined } from "@ant-design/icons";
import type { MessageInstance } from "antd/es/message/interface";

import {
  disable_user,
  enable_user,
  IUser,
  lock_user,
  unlock_user,
} from "../../../api/daffodil";
import Timestamp from "../../../components/Timestamp";
import { IError } from "../../../api";

const { Paragraph, Text } = Typography;

interface IProps {
  handleRefresh: () => void;
  name: string;
  item: IUser;
  messageApi: MessageInstance;
}

const Widget = ({ name, item, messageApi, handleRefresh }: IProps) => {
  const [isOpen, setOpen] = useState(false);
  const intl = useIntl();

  return (
    <>
      <Tooltip title={<FormattedMessage id="buttons.show" />}>
        <Button
          icon={<EyeOutlined />}
          shape="circle"
          size="small"
          onClick={() => {
            setOpen(true);
          }}
        />
      </Tooltip>
      <Modal
        title={name}
        open={isOpen}
        onOk={() => {
          setOpen(false);
        }}
        onCancel={() => {
          setOpen(false);
        }}
      >
        <Paragraph>
          <FormattedMessage id="models.user.sign-in-count" />
          :&nbsp;
          <Text strong>{item.signInCount}</Text>
        </Paragraph>
        <Paragraph>
          <FormattedMessage id="models.user.last-sign-in-at" />
          :&nbsp;
          <Text strong>
            {item.lastSignInAt && <Timestamp value={item.lastSignInAt} />}
          </Text>
        </Paragraph>
        <Paragraph>
          <FormattedMessage id="models.user.last-sign-in-ip" />
          :&nbsp;
          <Text strong>{item.lastSignInIp}</Text>
        </Paragraph>
        <Paragraph>
          <FormattedMessage id="models.user.current-sign-in-at" />
          :&nbsp;
          <Text strong>
            {item.currentSignInAt && <Timestamp value={item.currentSignInAt} />}
          </Text>
        </Paragraph>
        <Paragraph>
          <FormattedMessage id="models.user.current-sign-in-ip" />
          :&nbsp;
          <Text strong>{item.currentSignInIp}</Text>
        </Paragraph>

        <Space>
          {item.lockedAt ? (
            <Popconfirm
              title={<FormattedMessage id="flashes.are-you-sure" />}
              onConfirm={() => {
                unlock_user(item.id)
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
              <Button color="primary" variant="filled" size="small">
                <FormattedMessage id="buttons.unlock" />
              </Button>
            </Popconfirm>
          ) : (
            <Popconfirm
              title={<FormattedMessage id="flashes.are-you-sure" />}
              onConfirm={() => {
                lock_user(item.id)
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
              <Button color="danger" variant="filled" size="small">
                <FormattedMessage id="buttons.lock" />
              </Button>
            </Popconfirm>
          )}
          {item.deletedAt ? (
            <Popconfirm
              title={<FormattedMessage id="flashes.are-you-sure" />}
              onConfirm={() => {
                enable_user(item.id)
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
              <Button color="primary" variant="filled" size="small">
                <FormattedMessage id="buttons.enable" />
              </Button>
            </Popconfirm>
          ) : (
            <Popconfirm
              title={<FormattedMessage id="flashes.are-you-sure" />}
              onConfirm={() => {
                disable_user(item.id)
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
              <Button color="danger" variant="filled" size="small">
                <FormattedMessage id="buttons.disable" />
              </Button>
            </Popconfirm>
          )}
        </Space>
      </Modal>
    </>
  );
};
export default Widget;
