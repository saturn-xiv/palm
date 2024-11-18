import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { Typography, message, Button, Space, Popconfirm } from "antd";

import {
  enable_session,
  disable_session,
  ISession,
  index_session,
} from "../../../api/daffodil";
import { IError } from "../../../api";
import ShowUser from "../users/ShowUser";

interface IParams {
  key: string;
}

const Widget = () => {
  const intl = useIntl();
  const ref = useRef<ActionType>();
  const [messageApi, contextHolder] = message.useMessage();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.sessions.index.title" />
      </Typography.Title>
      {contextHolder}
      <ProTable<ISession, IParams>
        bordered
        actionRef={ref}
        params={{ key: "" }}
        search={false}
        columns={[
          {
            title: <FormattedMessage id="form.fields.id.label" />,
            dataIndex: "id",
            key: "id",
          },
          {
            title: <FormattedMessage id="form.fields.ip.label" />,
            dataIndex: "ip",
            key: "ip",
          },
          {
            title: <FormattedMessage id="form.fields.user.label" />,
            key: "user",
            render: (_, { realName, providerType }) => {
              return `${realName}(${providerType})`;
            },
          },
          {
            title: <FormattedMessage id="form.fields.uid.label" />,
            dataIndex: "uid",
            key: "uid",
          },
          {
            title: <FormattedMessage id="form.fields.expires-at.label" />,
            dataIndex: "expiresAt",
            key: "expiresAt",
          },
          {
            title: <FormattedMessage id="form.fields.created-at.label" />,
            dataIndex: "createdAt",
            key: "createdAt",
          },
          {
            title: <FormattedMessage id="buttons.manage" />,
            key: "resource",
            render: (_, { id, realName, detail, deletedAt }) => {
              return (
                <Space>
                  <ShowUser
                    messageApi={messageApi}
                    handleRefresh={() => ref.current?.reload()}
                    name={realName}
                    item={detail}
                  />
                  {deletedAt ? (
                    <Popconfirm
                      title={<FormattedMessage id="flashes.are-you-sure" />}
                      onConfirm={() => {
                        enable_session(id)
                          .then(() => {
                            messageApi
                              .success(
                                intl.formatMessage({
                                  id: "flashes.succeed",
                                })
                              )
                              .then(() => {
                                ref.current?.reload();
                              });
                          })
                          .catch((reason: IError[]) => {
                            messageApi.error(
                              reason.map((x) => x.message).join("\n")
                            );
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
                        disable_session(id)
                          .then(() => {
                            messageApi
                              .success(
                                intl.formatMessage({
                                  id: "flashes.succeed",
                                })
                              )
                              .then(() => {
                                ref.current?.reload();
                              });
                          })
                          .catch((reason: IError[]) => {
                            messageApi.error(
                              reason.map((x) => x.message).join("\n")
                            );
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
              );
            },
          },
        ]}
        rowKey="id"
        request={async (
          params: { key: string } & {
            pageSize?: number;
            current?: number;
          }
        ) => {
          if (params.pageSize && params.current) {
            const it = await index_session({
              page: params.current,
              size: params.pageSize,
            });
            return {
              data: it.items,
              success: true,
              total: it.pagination.total,
            };
          }
          return { data: [], success: false, total: 0 };
        }}
      />
    </>
  );
};

export default Widget;
