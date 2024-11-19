import type { ProFormInstance } from "@ant-design/pro-components";
import {
  ProForm,
  ProFormSelect,
  ProFormText,
} from "@ant-design/pro-components";
import { message, Switch, Typography } from "antd";
import { useEffect, useRef, useState } from "react";
import { FormattedMessage, useIntl } from "react-intl";

import {
  add_role_to_user,
  IUserSelectOption,
  policy_users,
  remove_role_from_user,
} from "../../../../api/daffodil";
import { IError } from "../../../../api";
import { NAME_MAX_LENGTH, NAME_MIN_LENGTH } from "../../../../components";

interface IFormValue {
  user: number;
  role: string;
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
        <FormattedMessage id="pages.admin.policies.roles.form.title" />
      </Typography.Title>
      {contextHolder}
      <ProForm<IFormValue>
        onFinish={async (values) => {
          if (enable) {
            add_role_to_user(values.user, values.role)
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
            remove_role_from_user(values.user, values.role)
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
        <ProFormText
          name="role"
          width="md"
          label={<FormattedMessage id="form.fields.role.label" />}
          rules={[
            { required: true },
            { min: NAME_MIN_LENGTH, max: NAME_MAX_LENGTH },
          ]}
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
