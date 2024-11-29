import { Card, Space } from "antd";
import { FormattedMessage } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";

import { IPage } from "../../../api/cms";
import Summary from "../../../components/Summary";
import Timestamp from "../../../components/Timestamp";
import EditForm from "./Edit";

interface IProps {
  item: IPage;
  messageApi: MessageInstance;
  handleReload: () => void;
}

const { Meta } = Card;

const Widget = ({ item, messageApi, handleReload }: IProps) => {
  return (
    <Card
      title={item.title}
      extra={
        <EditForm
          item={item}
          messageApi={messageApi}
          handleReload={handleReload}
        />
      }
    >
      <p>
        <Summary wordwrap={320} html={item.body} />
      </p>
      <Meta
        title={item.slug}
        description={
          <Space>
            <Timestamp value={item.updatedAt} />
            <FormattedMessage id={`languages.${item.lang}`} />
          </Space>
        }
      />
    </Card>
  );
};

export default Widget;
