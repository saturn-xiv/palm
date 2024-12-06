import { Typography } from "antd";
import { FormattedMessage } from "react-intl";

import Upload from "../../../attachments/Upload";

const Widget = () => {
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.site.favicon.title" />
      </Typography.Title>
      <Upload public action="/api/admin/site/favicon" />
    </>
  );
};
export default Widget;
