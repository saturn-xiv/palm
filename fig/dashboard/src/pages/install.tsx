import type { ProFormInstance } from "@ant-design/pro-components";
import { StepsForm, ProFormText } from "@ant-design/pro-components";
import { message, Typography } from "antd";
import { useRef } from "react";
import { FormattedMessage, useIntl } from "react-intl";
import { useNavigate } from "react-router-dom";

import { guess_timezone } from "../utils";
import { install } from "../api/daffodil";
import { IError } from "../api";
import { SIGN_IN_PATH } from "../reducers/current-user";
import { IFormValue as IUserFormValue } from "./users/sign-up";
import {
  EMAIL_MAX_LENGTH,
  EMAIL_MIN_LENGTH,
  NAME_MAX_LENGTH,
  NAME_MIN_LENGTH,
  PASSWORD_MAX_LENGTH,
  PASSWORD_MIN_LENGTH,
} from "../components";

interface ISiteFormValue {
  title: string;
  subhead: string;
  description: string;
  copyright: string;
}

const SiteForm = () => {
  const intl = useIntl();
  const formRef = useRef<ProFormInstance<ISiteFormValue>>();
  return (
    <StepsForm.StepForm<ISiteFormValue>
      name="site-info"
      title={intl.formatMessage({ id: "pages.install.site-info.title" })}
      stepProps={{
        description: intl.formatMessage({
          id: "pages.install.site-info.description",
        }),
      }}
      onFinish={async () => {
        return true;
      }}
      formRef={formRef}
      formKey="install.site-info"
      request={async () => {
        return {
          title: "Demo site title",
          subhead: "Demo site subhead",
          description: "Demo information",
          copyright: `~ ${new Date().getFullYear()}`,
        };
      }}
      autoFocusFirstInput
    >
      <ProFormText
        name="title"
        width="md"
        label={<FormattedMessage id="form.fields.title.label" />}
      />
      <ProFormText
        name="subhead"
        width="md"
        label={
          <FormattedMessage id="pages.admin.site.base.form.subhead.label" />
        }
      />
      <ProFormText
        name="description"
        width="md"
        label={<FormattedMessage id="form.fields.description.label" />}
      />
      <ProFormText
        name="copyright"
        width="md"
        label={
          <FormattedMessage id="pages.admin.site.base.form.copyright.label" />
        }
        rules={[{ required: true }]}
      />
    </StepsForm.StepForm>
  );
};

const UserForm = () => {
  const intl = useIntl();
  const formRef = useRef<ProFormInstance<IUserFormValue>>();
  return (
    <StepsForm.StepForm<IUserFormValue>
      name="site.info"
      title={intl.formatMessage({ id: "pages.install.administrator.title" })}
      stepProps={{
        description: intl.formatMessage({
          id: "pages.install.administrator.description",
        }),
      }}
      onFinish={async () => {
        return true;
      }}
      formRef={formRef}
      formKey="administrator"
      request={async () => {
        return {
          realName: "",
          nickname: "",
          email: "",
          password: "",
          passwordConfirmation: "",
        };
      }}
      autoFocusFirstInput
    >
      <ProFormText
        name="realName"
        width="md"
        label={<FormattedMessage id="form.fields.real-name.label" />}
        rules={[
          { required: true },
          { min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH },
        ]}
      />
      <ProFormText
        name="email"
        width="md"
        label={<FormattedMessage id="form.fields.email.label" />}
        rules={[
          { required: true },
          { min: EMAIL_MIN_LENGTH, max: EMAIL_MAX_LENGTH },
        ]}
      />
      <ProFormText
        name="nickname"
        width="md"
        label={<FormattedMessage id="form.fields.nickname.label" />}
        rules={[
          { required: true },
          { min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH },
        ]}
      />
      <ProFormText.Password
        name="password"
        width="md"
        label={<FormattedMessage id="form.fields.password.label" />}
        rules={[
          { required: true },
          { min: PASSWORD_MIN_LENGTH, max: PASSWORD_MAX_LENGTH },
        ]}
      />
      <ProFormText.Password
        name="passwordConfirmation"
        width="md"
        label={
          <FormattedMessage id="form.fields.password-confirmation.label" />
        }
        rules={[
          { required: true },
          ({ getFieldValue }) => ({
            validator(_, value) {
              if (!value || getFieldValue("password") === value) {
                return Promise.resolve();
              }
              return Promise.reject(
                new Error(
                  intl.formatMessage({
                    id: "form.fields.password-confirmation.errors.not-match",
                  })
                )
              );
            },
          }),
        ]}
      />
    </StepsForm.StepForm>
  );
};

interface IFormValue extends IUserFormValue, ISiteFormValue {}

const Widget = () => {
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const navigate = useNavigate();
  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.install.title" />
      </Typography.Title>
      {contextHolder}
      <StepsForm<IFormValue>
        onFinish={async (values) => {
          install(
            {
              title: values.title,
              subhead: values.subhead,
              description: values.description,
              copyright: values.copyright,
            },
            {
              realName: values.realName,
              nickname: values.nickname,
              email: values.email,
              password: values.password,
              timezone: guess_timezone(),
            }
          )
            .then(() => {
              messageApi
                .success(intl.formatMessage({ id: "flashes.succeed" }))
                .then(() => {
                  navigate(SIGN_IN_PATH);
                });
            })
            .catch((reason: IError[]) => {
              messageApi.error(reason.map((x) => x.message).join("\n"));
            });
        }}
        stepsProps={{
          direction: "vertical",
        }}
      >
        <SiteForm />
        <UserForm />
      </StepsForm>
    </>
  );
};

export default Widget;
