import { List, Typography } from "antd";
import { FormattedMessage } from "react-intl";

const Widget = () => {
  return (
    <List
      header={
        <Typography.Title level={3}>
          <FormattedMessage id="pages.admin.policies.roles.built-in.title" />
        </Typography.Title>
      }
      bordered
      dataSource={["cms.manager", "bbs.manager", "accounting.member"]}
      renderItem={(it, id) => (
        <List.Item key={id}>
          <Typography.Text>{it}</Typography.Text>
        </List.Item>
      )}
    />
  );
};

export default Widget;
