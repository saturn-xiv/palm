import { List, Typography } from "antd";
import { FormattedMessage } from "react-intl";

import { IUserSelectOption } from "../../../../api/daffodil";

interface IProps {
  items: IUserSelectOption[];
}

const Widget = ({ items }: IProps) => {
  return (
    <List
      size="small"
      header={
        <Typography.Title level={4}>
          <FormattedMessage id="pages.admin.policies.administrators.user-list.title" />
        </Typography.Title>
      }
      bordered
      dataSource={items}
      renderItem={(x, i) => <List.Item key={i}>{x.label}</List.Item>}
      pagination={{}}
    />
  );
};

export default Widget;
