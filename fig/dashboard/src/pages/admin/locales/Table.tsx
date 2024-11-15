import { ActionType, ProTable } from "@ant-design/pro-components";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { Typography, Popconfirm, message, Button, Tooltip, Space } from "antd";
import { DeleteOutlined } from "@ant-design/icons";

import { destroy_locale, ILocale, index_locale } from "../../../api/daffodil";
import { IError } from "../../../api";
import Form from "./Form";

interface IParams {
  key: string;
}

const Widget = () => {
  const intl = useIntl();
  const ref = useRef<ActionType>();
  const [messageApi, contextHolder] = message.useMessage();
  const handleRefresh = () => ref.current?.reload();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.locales.index.title" />
      </Typography.Title>
      {contextHolder}
      <ProTable<ILocale, IParams>
        bordered
        actionRef={ref}
        params={{ key: "" }}
        search={false}
        columns={[
          {
            title: <FormattedMessage id="form.fields.lang.label" />,
            dataIndex: "lang",
            key: "lang",
          },
          {
            title: <FormattedMessage id="form.fields.code.label" />,
            dataIndex: "code",
            key: "code",
          },
          {
            title: <FormattedMessage id="form.fields.message.label" />,
            dataIndex: "message",
            key: "message",
          },
          {
            title: <FormattedMessage id="form.fields.updated-at.label" />,
            dataIndex: "updatedAt",
            key: "updatedAt",
          },
          {
            title: (
              <Space>
                <FormattedMessage id="buttons.manage" />
                <Form messageApi={messageApi} handleRefresh={handleRefresh} />
              </Space>
            ),
            key: "resource",
            render: (_, { id, code, lang, message }) => {
              return (
                <Space>
                  <Form
                    messageApi={messageApi}
                    handleRefresh={handleRefresh}
                    item={{ code, lang, message }}
                  />
                  <Popconfirm
                    title={<FormattedMessage id="flashes.are-you-sure" />}
                    onConfirm={() => {
                      destroy_locale(id)
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
                    <Tooltip title={<FormattedMessage id="buttons.delete" />}>
                      <Button
                        danger
                        icon={<DeleteOutlined />}
                        shape="circle"
                        size="small"
                      />
                    </Tooltip>
                  </Popconfirm>
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
            const it = await index_locale({
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
