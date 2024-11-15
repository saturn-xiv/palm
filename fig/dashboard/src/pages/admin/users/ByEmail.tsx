import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { Typography, message, Button, Space, Popconfirm } from "antd";

import {
  confirm_email_user,
  enable_email_user,
  disable_email_user,
  IEmailUser,
  index_email_user,
} from "../../../api/daffodil";
import { IError } from "../../../api";
import ShowUser from "./ShowUser";

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
        <FormattedMessage id="pages.admin.users.by-email.title" />
      </Typography.Title>
      {contextHolder}
      <ProTable<IEmailUser, IParams>
        bordered
        actionRef={ref}
        params={{ key: "" }}
        search={false}
        columns={[
          {
            title: <FormattedMessage id="form.fields.nickname.label" />,
            dataIndex: "nickname",
            key: "nickname",
          },
          {
            title: <FormattedMessage id="form.fields.user.label" />,
            key: "user",
            render: (_, { realName, email }) => {
              return `${realName}<${email}>`;
            },
          },
          {
            title: <FormattedMessage id="form.fields.updated-at.label" />,
            dataIndex: "updatedAt",
            key: "updatedAt",
          },
          {
            title: <FormattedMessage id="buttons.manage" />,
            key: "resource",
            render: (_, { id, realName, detail, confirmedAt, deletedAt }) => {
              return (
                <Space>
                  <ShowUser
                    messageApi={messageApi}
                    handleRefresh={() => ref.current?.reload()}
                    name={realName}
                    item={detail}
                  />
                  {!confirmedAt && (
                    <Popconfirm
                      title={<FormattedMessage id="flashes.are-you-sure" />}
                      onConfirm={() => {
                        confirm_email_user(id)
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
                      <Button color="default" variant="filled" size="small">
                        <FormattedMessage id="buttons.verify" />
                      </Button>
                    </Popconfirm>
                  )}

                  {deletedAt ? (
                    <Popconfirm
                      title={<FormattedMessage id="flashes.are-you-sure" />}
                      onConfirm={() => {
                        enable_email_user(id)
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
                        disable_email_user(id)
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
            const it = await index_email_user({
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
