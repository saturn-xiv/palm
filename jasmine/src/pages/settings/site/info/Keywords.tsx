import { useState } from "react";
import { ProForm, ProFormSelect } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { IErrorMessage } from "../../../../api/graphql";
import { fetch_layout, set_site_keywords } from "../../../../api/site";

interface IProps {
  handleRefresh: () => void;
}
interface IForm {
  items: string[];
}

const Widget = ({ handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const [items, setItems] = useState<string[]>([]);

  return (
    <Card
      title={<FormattedMessage id="settings.site.info.keywords.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          set_site_keywords(values.items)
            .then(() => {
              messageApi.success(intl.formatMessage({ id: "flashes.succeed" }));
              handleRefresh();
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        request={async () => {
          const it = await fetch_layout();
          setItems(it.layout.keywords);
          return {
            items: it.layout.keywords,
          };
        }}
      >
        <ProFormSelect
          mode="tags"
          options={items.map((x: string) => {
            return {
              value: x,
              label: x,
            };
          })}
          width="md"
          cacheForSwr
          name="items"
          label={<FormattedMessage id="form.fields.keywords.label" />}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;
