import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { Typography, message, Button, Space, Popconfirm } from "antd";

import {
  close_leave_word,
  enable_leave_word,
  disable_leave_word,
  ILeaveWord,
  index_leave_word,
} from "../../../api/daffodil";
import { IError } from "../../../api";

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
        <FormattedMessage id="pages.admin.leave-words.index.title" />
      </Typography.Title>
      {contextHolder}
      <ProTable<ILeaveWord, IParams>
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
            title: <FormattedMessage id="form.fields.body.label" />,
            dataIndex: "body",
            key: "body",
          },
          {
            title: <FormattedMessage id="form.fields.status.label" />,
            dataIndex: "status",
            key: "status",
          },
          {
            title: <FormattedMessage id="form.fields.updated-at.label" />,
            dataIndex: "updatedAt",
            key: "updatedAt",
          },
          {
            title: <FormattedMessage id="buttons.manage" />,
            key: "resource",
            render: (_, { id, status, deletedAt }) => {
              return (
                <Space>
                  {status === "Pending" && (
                    <Popconfirm
                      title={<FormattedMessage id="flashes.are-you-sure" />}
                      onConfirm={() => {
                        close_leave_word(id)
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
                        <FormattedMessage id="buttons.close" />
                      </Button>
                    </Popconfirm>
                  )}

                  {deletedAt ? (
                    <Popconfirm
                      title={<FormattedMessage id="flashes.are-you-sure" />}
                      onConfirm={() => {
                        enable_leave_word(id)
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
                        disable_leave_word(id)
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
            const it = await index_leave_word({
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
