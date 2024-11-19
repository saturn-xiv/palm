import { Button, List, Space, Typography } from "antd";
import { FormattedMessage } from "react-intl";

import { IUserRoleRelation } from "../../../../api/daffodil";

const { Text } = Typography;

interface IProps {
  items: IUserRoleRelation[];
}

const Widget = ({ items }: IProps) => {
  return (
    <List
      size="small"
      header={
        <Typography.Title level={4}>
          <FormattedMessage id="pages.admin.policies.roles.relation-list.title" />
        </Typography.Title>
      }
      bordered
      dataSource={items}
      renderItem={(x, i) => (
        <List.Item key={i}>
          <Text mark>{x.role}</Text>
          <Space>
            {x.users.map((y) => (
              <Button type="text" key={y.id}>
                {y.label}
              </Button>
            ))}
          </Space>
        </List.Item>
      )}
      pagination={{}}
    />
  );
};

export default Widget;
