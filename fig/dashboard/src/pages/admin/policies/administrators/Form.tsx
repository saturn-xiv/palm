import type { ProFormInstance } from "@ant-design/pro-components";
import { ProForm, ProFormSelect } from "@ant-design/pro-components";
import { message, Switch, Typography } from "antd";
import { useEffect, useRef, useState } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import {
  disable_administrator,
  enable_administrator,
  IUserSelectOption,
  policy_users,
} from "../../../../api/daffodil";
import { IError } from "../../../../api";

interface IFormValue {
  user: number;
}
interface IProps {
  handleRefresh: () => void;
}
const Widget = ({ handleRefresh }: IProps) => {
  const [users, setUsers] = useState<IUserSelectOption[]>([]);
  const [enable, setEnable] = useState<boolean>(false);
  const intl = useIntl();
  const [messageApi, contextHolder] = message.useMessage();
  const formRef = useRef<ProFormInstance<IFormValue>>();
  useEffect(() => {
    policy_users().then((res) => {
      setUsers(res);
    });
  }, []);

  return (
    <>
      <Typography.Title level={3}>
        <FormattedMessage id="pages.admin.policies.administrators.form.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          if (enable) {
            enable_administrator(values.user)
              .then(() => {
                messageApi
                  .success(intl.formatMessage({ id: "flashes.succeed" }))
                  .then(() => {
                    handleRefresh();
                  });
              })
              .catch((reason: IError[]) => {
                messageApi.error(reason.map((x) => x.message).join("\n"));
              });
          } else {
            disable_administrator(values.user)
              .then(() => {
                messageApi
                  .success(intl.formatMessage({ id: "flashes.succeed" }))
                  .then(() => {
                    handleRefresh();
                  });
              })
              .catch((reason: IError[]) => {
                messageApi.error(reason.map((x) => x.message).join("\n"));
              });
          }
        }}
        formRef={formRef}
        formKey="policies.administrator"
        autoFocusFirstInput
      >
        <ProFormSelect
          width="md"
          name="user"
          label={<FormattedMessage id="form.fields.user.label" />}
          options={users.map((x) => {
            return {
              label: x.label,
              value: x.id,
            };
          })}
          rules={[{ required: true }]}
        />
        <Switch
          style={{
            marginBlockEnd: 16,
          }}
          checked={enable}
          checkedChildren={<FormattedMessage id="buttons.enable" />}
          unCheckedChildren={<FormattedMessage id="buttons.disable" />}
          onChange={setEnable}
        />
      </ProForm>
    </>
  );
};

export default Widget;
