import { Card, List } from "antd";
import { FormattedMessage } from "react-intl";

interface IProps {
  languages: string[];
}
const Widget = ({ languages }: IProps) => {
  return (
    <Card
      title={<FormattedMessage id="settings.site.seo.sitemap.title" />}
      hoverable
    >
      <List<string>
        bordered
        dataSource={["/robots.txt", "/sitemap.xml"].concat(
          languages.map((x) => `/sitemap/${x}.xml`)
        )}
        renderItem={(item) => (
          <List.Item
            onClick={() => {
              window.open(item, "_blank")?.focus();
            }}
          >
            {item}
          </List.Item>
        )}
      />
    </Card>
  );
};

export default Widget;
