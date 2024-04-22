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
        renderItem={(item) => (
          <List.Item
            onClick={() => {
              window.open(`/${item}/rss.xml`, "_blank")?.focus();
            }}
          >
            <FormattedMessage id={`languages.${item}`} />
          </List.Item>
        )}
      />
    </Card>
  );
};

export default Widget;
