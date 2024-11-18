import { Tabs } from "antd";
import { FormattedMessage } from "react-intl";

import Seo from "./seo";
import Info from "./info";
import Maintenance from "./maintenance";
import Status from "./status";
import China from "./china";

const Widget = () => {
  return (
    <Tabs
      defaultActiveKey="status"
      items={[
        {
          key: "info",
          label: <FormattedMessage id="pages.admin.site.tabs.info.label" />,
          children: <Info />,
        },
        {
          key: "china",
          label: <FormattedMessage id="pages.admin.site.tabs.china.label" />,
          children: <China />,
        },
        {
          key: "seo",
          label: <FormattedMessage id="pages.admin.site.tabs.seo.label" />,
          children: <Seo />,
        },
        {
          key: "maintenance",
          label: (
            <FormattedMessage id="pages.admin.site.tabs.maintenance.label" />
          ),
          children: <Maintenance />,
        },
        {
          key: "status",
          label: <FormattedMessage id="pages.admin.site.tabs.status.label" />,
          children: <Status />,
        },
      ]}
    />
  );
};

export default Widget;
