import { message, List, Typography, Space, Button } from "antd";
import { FormattedMessage } from "react-intl";

import { ICategory } from "../../../api/daffodil";
import Create from "./Create";
import Append from "./Append";
import Delete from "./Delete";
import Edit from "./Edit";
import { ROOT } from "./Tree";

interface IProps {
  handleRefresh: () => void;
  handleShow: (it: ICategory) => void;
  nodes: ICategory[];
}
const Widget = ({ nodes, handleShow, handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();

  return (
    <>
      {contextHolder}
      <List
        size="small"
        header={
          <Typography.Title level={4}>
            <FormattedMessage id="pages.admin.categories.list.title" />
          </Typography.Title>
        }
        footer={
          <Space>
            <Create
              nodes={nodes}
              messageApi={messageApi}
              handleRefresh={handleRefresh}
            />
            <Append
              nodes={nodes}
              messageApi={messageApi}
              handleRefresh={handleRefresh}
            />
          </Space>
        }
        bordered
        dataSource={nodes}
        renderItem={(x) => (
          <List.Item key={x.id}>
            <Space>
              <Button
                type="text"
                onClick={() => {
                  handleShow(x);
                }}
              >
                {x.code}({x.left},{x.right})
              </Button>
              {x.code === ROOT ? (
                <></>
              ) : (
                <>
                  <Edit
                    item={x}
                    messageApi={messageApi}
                    handleRefresh={handleRefresh}
                  />
                  <Delete
                    item={x}
                    messageApi={messageApi}
                    handleRefresh={handleRefresh}
                  />
                </>
              )}
            </Space>
          </List.Item>
        )}
        pagination={{}}
      />
    </>
  );
};

export default Widget;
