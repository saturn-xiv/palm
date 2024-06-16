import { Popconfirm, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import { DeleteOutlined } from "@ant-design/icons";

import { IErrorMessage } from "../../../api/graphql";
import { ILocale, destroy as destroy_locale } from "../../../api/locales";

interface IProps {
  item: ILocale;
  handleRefresh: () => void;
}

const Widget = ({ item, handleRefresh }: IProps) => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  return (
    <Popconfirm
      title={<FormattedMessage id="flashes.are-you-sure" />}
      description={
        <FormattedMessage
          id="settings.locales.destroy.confirm.description"
          values={{
            code: item.code,
            lang: intl.formatMessage({ id: `languages.${item.lang}` }),
          }}
        />
      }
      onConfirm={() => {
        destroy_locale(item.id)
          .then(() => {
            messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
            handleRefresh();
          })
          .catch((reason: IErrorMessage[]) => {
            message.error(reason.map((x) => x.message).join("\n"));
          });
      }}
      okText={<FormattedMessage id="buttons.ok" />}
      cancelText={<FormattedMessage id="buttons.cancel" />}
    >
      {contextHolder}
      <DeleteOutlined />
    </Popconfirm>
  );
};

export default Widget;
