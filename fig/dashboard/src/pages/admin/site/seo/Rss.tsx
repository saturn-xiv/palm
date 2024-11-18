import { List, Button, Typography } from "antd";

import { available_languages } from "../../../../i18n";

const Widget = () => {
  return (
    <List
      header={<Typography.Title level={3}>RSS</Typography.Title>}
      footer={
        <Button
          key="help"
          type="link"
          onClick={(e) => {
            e.preventDefault();
            window
              .open("https://rss.com/blog/how-do-rss-feeds-work/", "_blank")
              ?.focus();
          }}
        >
          How Do RSS Feeds Work?
        </Button>
      }
      bordered
      dataSource={available_languages.map((x) => `/${x}/rss.xml`)}
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
