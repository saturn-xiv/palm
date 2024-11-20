import { message, List, Typography, Space } from "antd";
import { FormattedMessage } from "react-intl";
import { useEffect, useState } from "react";

import { index_tag, ITag } from "../../../api/daffodil";
import Form from "./Form";
import Delete from "./Delete";

const Widget = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const [items, setItems] = useState<ITag[]>([]);
  const handleRefresh = () => {
    index_tag().then((res) => setItems(res));
  };
  useEffect(() => {
    handleRefresh();
  }, []);
  return (
    <>
      {contextHolder}
      <List
        size="small"
        header={
          <Typography.Title level={4}>
            <FormattedMessage id="pages.admin.tags.list.title" />
          </Typography.Title>
        }
        footer={<Form messageApi={messageApi} handleRefresh={handleRefresh} />}
        bordered
        dataSource={items}
        renderItem={(x) => (
          <List.Item key={x.id}>
            <Space>
              {x.code}
              <Form
                item={x}
                messageApi={messageApi}
                handleRefresh={handleRefresh}
              />
              <Delete
                item={x}
                messageApi={messageApi}
                handleRefresh={handleRefresh}
              />
            </Space>
          </List.Item>
        )}
        pagination={{}}
      />
    </>
  );
};

export default Widget;
