import { useState } from "react";
import {
  ProForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { Card, message } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { useAppDispatch, useAppSelector } from "../../../hooks";
import { updateProfile } from "../../../reducers/current-user";
import { siteInfo as selectSiteInfo } from "../../../reducers/site-info";
import { IErrorMessage } from "../../../api/graphql";
import {
  update_profile,
  current_email_user,
  current_user,
  is_email_user,
} from "../../../api/users";
import {
  REAL_NAME_MAX_LENGTH,
  REAL_NAME_MIN_LENGTH,
} from "../../users/sign-up";
import { timezones } from "../../../utils";

interface IForm {
  email?: string;
  avatar: string;
  nickname?: string;
  real_name: string;
  lang: string;
  timezone: string;
}

const Widget = () => {
  const [providerType, setProviderType] = useState<string | undefined>();
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  const dispatch = useAppDispatch();
  const site_info = useAppSelector(selectSiteInfo);
  return (
    <Card
      title={<FormattedMessage id="personal.profile.update.title" />}
      hoverable
    >
      {contextHolder}
      <ProForm<IForm>
        onFinish={async (values) => {
          update_profile(
            values.real_name,
            values.avatar,
            values.lang,
            values.timezone
          )
            .then(() => {
              messageApi.success(
                intl.formatMessage({ id: "users.confirm.by-email.succeed" })
              );
              dispatch(
                updateProfile({
                  realName: values.real_name,
                  avatar: values.avatar,
                  lang: values.lang,
                  timezone: values.timezone,
                })
              );
            })
            .catch((reason: IErrorMessage[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        request={async () => {
          const it = await current_user();
          const ret: IForm = {
            real_name: it.name,
            avatar: it.avatar,
            lang: it.lang,
            timezone: it.timezone,
          };
          if (is_email_user(it.providerType)) {
            const it = await current_email_user();
            ret.email = it.email;
            ret.nickname = it.nickname;
          }
          setProviderType(it.providerType);
          return ret;
        }}
      >
        <ProFormText
          width="md"
          name="real_name"
          label={<FormattedMessage id="form.fields.real-name.label" />}
          rules={[
            {
              required: true,
              min: REAL_NAME_MIN_LENGTH,
              max: REAL_NAME_MAX_LENGTH,
            },
          ]}
        />
        {is_email_user(providerType) && (
          <ProFormText
            width="md"
            name="nickname"
            disabled
            label={<FormattedMessage id="form.fields.nickname.label" />}
          />
        )}

        {is_email_user(providerType) && (
          <ProFormText
            width="md"
            name="email"
            disabled
            label={<FormattedMessage id="form.fields.email.label" />}
          />
        )}

        <ProFormText
          width="md"
          name="avatar"
          label={<FormattedMessage id="form.fields.avatar.label" />}
          rules={[{ required: true, type: "url" }]}
        />
        <ProFormSelect
          options={timezones().map((x) => {
            return { value: x, label: x };
          })}
          width="md"
          cacheForSwr
          name="timezone"
          label={<FormattedMessage id="form.fields.timezone.label" />}
        />
        <ProFormSelect
          options={site_info.languages.map((x: string) => {
            return {
              value: x,
              label: intl.formatMessage({ id: `languages.${x}` }),
            };
          })}
          width="md"
          cacheForSwr
          name="lang"
          label={<FormattedMessage id="form.fields.language.label" />}
        />
      </ProForm>
    </Card>
  );
};

export default Widget;
