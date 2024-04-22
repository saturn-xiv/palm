import {
  ProForm,
  ProFormText,
  ProFormTextArea,
} from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { IErrorMessage } from "../../../../api/graphql";
import { fetch_layout, set_site_info } from "../../../../api/camelia";
import { get as get_locale } from "../../../../locales";

interface IProps {
  handleRefresh: () => void;
}
interface IForm {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
}

const Widget = ({ handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();

  return (
    <Card
      title={<FormattedMessage id="settings.site.info.base.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          set_site_info(
            values.title,
            values.subhead,
            values.description,
            values.copyright
          )
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
            title: it.siteInfo.title,
            subhead: it.siteInfo.subhead,
            description: it.siteInfo.description,
            copyright: it.siteInfo.copyright,
          };
        }}
      >
        <ProFormText
          width="md"
          name="title"
          label={<FormattedMessage id="form.fields.title.label" />}
          rules={[{ required: true, min: 1, max: 63 }]}
        />
        <ProFormText
          width="md"
          name="subhead"
          label={<FormattedMessage id="form.fields.subhead.label" />}
          rules={[{ required: true, min: 1, max: 31 }]}
        />
        <ProFormTextArea
          width="md"
          name="description"
          label={<FormattedMessage id="form.fields.description.label" />}
          rules={[{ required: true, min: 1, max: 511 }]}
        />
        <ProFormText
          width="md"
          name="copyright"
          label={
            <FormattedMessage id="settings.site.info.base.fields.copyright.label" />
          }
          rules={[{ required: true, min: 1, max: 31 }]}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;
