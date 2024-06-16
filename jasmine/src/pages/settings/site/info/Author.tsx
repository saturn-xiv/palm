import { ProForm, ProFormText } from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { IErrorMessage } from "../../../../api/graphql";
import { fetch_layout, set_site_authors } from "../../../../api/site";
import {
  EMAIL_MAX_LENGTH,
  REAL_NAME_MAX_LENGTH,
  REAL_NAME_MIN_LENGTH,
} from "../../../users/sign-up";

interface IProps {
  handleRefresh: () => void;
}
interface IForm {
  name: string;
  email: string;
}

const Widget = ({ handleRefresh }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();

  return (
    <Card
      title={<FormattedMessage id="settings.site.info.authors.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          set_site_authors([{ name: values.name, email: values.email }])
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
          return {
            name: it.layout.author?.name || "",
            email: it.layout.author?.email || "",
          };
        }}
      >
        <ProFormText
          width="md"
          name="name"
          label={<FormattedMessage id="form.fields.username.label" />}
          rules={[
            {
              required: true,
              min: REAL_NAME_MIN_LENGTH,
              max: REAL_NAME_MAX_LENGTH,
            },
          ]}
        />
        <ProFormText
          width="md"
          name="email"
          label={<FormattedMessage id="form.fields.email.label" />}
          rules={[{ required: true, type: "email", max: EMAIL_MAX_LENGTH }]}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;
