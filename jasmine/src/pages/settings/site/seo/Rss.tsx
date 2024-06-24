import { Card, List } from "antd";
import { FormattedMessage } from "react-intl";

interface IProps {
  languages: string[];
}
const Widget = ({ languages }: IProps) => {
  return (
    <Card
      title={<FormattedMessage id="settings.site.seo.rss.title" />}
      hoverable
    >
      <List<string>
        bordered
        dataSource={languages}
        renderItem={(x) => (
          <List.Item
            onClick={() => {
              window.open(`/rss/${x}.xml`, "_blank")?.focus();
            }}
          >
            <FormattedMessage id={`languages.${x}`} />
          </List.Item>
        )}
      />
    </Card>
  );
};

export default Widget;
