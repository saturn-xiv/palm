import { List, Button, Typography } from "antd";

import { available_languages } from "../../../../i18n";

const Widget = () => {
  return (
    <List
      header={<Typography.Title level={3}>Sitemap</Typography.Title>}
      footer={
        <Button
          key="help"
          type="link"
          onClick={(e) => {
            e.preventDefault();
            window
              .open("https://www.sitemaps.org/protocol.html", "_blank")
              ?.focus();
          }}
        >
          Sitemaps XML format
        </Button>
      }
      bordered
      dataSource={["/sitemap.xml"].concat(
        available_languages.map((x) => `/${x}/sitemap.xml`)
      )}
      renderItem={(x) => (
        <List.Item key={x}>
          <Button
            type="text"
            onClick={(e) => {
              e.preventDefault();
              window.open(x, "_blank")?.focus();
            }}
          >
            {x}
          </Button>
        </List.Item>
      )}
    />
  );
};

export default Widget;
