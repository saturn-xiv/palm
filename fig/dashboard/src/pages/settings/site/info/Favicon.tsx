import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { IErrorMessage } from "../../../../api/graphql";
import { fetch_layout, set_site_favicon } from "../../../../api/camelia";
import { get as get_locale } from "../../../../locales";

interface IProps {
  handleRefresh: () => void;
}
interface IForm {
  url: string;
}

const Widget = ({ handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();

  return (
    <Card
      title={<FormattedMessage id="settings.site.info.favicon.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          set_site_favicon(values.url)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
              handleRefresh();
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        request={async () => {
          const it = await fetch_layout(get_locale());
          return {
            url: it.siteInfo.favicon,
          };
        }}
      >
        <ProFormText
          width="md"
          name="url"
          label={<FormattedMessage id="form.fields.url.label" />}
          rules={[{ required: true }]}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;
