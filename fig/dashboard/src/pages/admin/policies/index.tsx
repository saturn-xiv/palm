import { Tabs } from "antd";
import { FormattedMessage } from "react-intl";

import Roles from "./roles";
import Administrators from "./administrators";

const Widget = () => {
  return (
    <Tabs
      defaultActiveKey="roles"
      items={[
        {
          key: "roles",
          label: (
            <FormattedMessage id="pages.admin.policies.tabs.roles.label" />
          ),
          children: <Roles />,
        },
        {
          key: "administrators",
          label: (
            <FormattedMessage id="pages.admin.policies.tabs.administrators.label" />
          ),
          children: <Administrators />,
        },
      ]}
    />
  );
};

export default Widget;
