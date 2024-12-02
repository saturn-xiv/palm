import { PlusOutlined, EditOutlined } from "@ant-design/icons";
import { ModalForm, ProFormText } from "@ant-design/pro-components";
import { Button, Form } from "antd";
import { FormattedMessage, useIntl } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";

import { IPostalAddress, IPostalAddressFormValue } from "../../../api/daffodil";
import { IError } from "../../../api";
import { URL_MAX_LENGTH, URL_MIN_LENGTH } from "../..";

interface IProps {
  item?: IPostalAddress;
  messageApi: MessageInstance;
  handleSave: (it: IPostalAddressFormValue) => Promise<void>;
  handleReload: () => void;
  title: string;
}

const Widget = ({
  title,
  item,
  messageApi,
  handleSave,
  handleReload,
}: IProps) => {
  const [form] = Form.useForm<IPostalAddressFormValue>();
  const intl = useIntl();

  return (
    <ModalForm<IPostalAddressFormValue>
      title={title}
      trigger={
        <Button
          icon={item ? <EditOutlined /> : <PlusOutlined />}
          size="small"
          type="dashed"
        />
      }
      form={form}
      autoFocusFirstInput
      modalProps={{
        destroyOnClose: true,
      }}
      request={async () => {
        return {
          unit: item?.unit,
          building: item?.building,
          street: item?.street || "",
          city: item?.city || "",
          province: item?.province || "",
          country: item?.country || "",
          zipCode: item?.zipCode || "",
          passcode: item?.passcode,
          googleMap: item?.googleMap,
          aMap: item?.aMap,
        };
      }}
      onFinish={async (values) => {
        const ok = await handleSave(values)
          .then(async () => {
            await messageApi
              .success(intl.formatMessage({ id: "flashes.succeed" }))
              .then(() => {
                handleReload();
              });
            return true;
          })
          .catch((reason: IError[]) => {
            messageApi.error(reason.map((x) => x.message).join("\n"));
            return false;
          });
        return ok;
      }}
    >
      <ProFormText
        name="unit"
        label={
          <FormattedMessage id="components.postal.address-form.fields.unit.label" />
        }
        rules={[{ min: 1, max: 7 }]}
      />
      <ProFormText
        name="building"
        label={
          <FormattedMessage id="components.postal.address-form.fields.building.label" />
        }
        rules={[{ min: 1, max: 31 }]}
      />
      <ProFormText
        name="street"
        label={
          <FormattedMessage id="components.postal.address-form.fields.street.label" />
        }
        rules={[{ required: true }, { min: 1, max: 127 }]}
      />
      <ProFormText
        name="city"
        label={
          <FormattedMessage id="components.postal.address-form.fields.city.label" />
        }
        rules={[{ required: true }, { min: 1, max: 127 }]}
      />
      <ProFormText
        name="province"
        label={
          <FormattedMessage id="components.postal.address-form.fields.province.label" />
        }
        rules={[{ required: true }, { min: 1, max: 127 }]}
      />
      <ProFormText
        name="country"
        label={
          <FormattedMessage id="components.postal.address-form.fields.country.label" />
        }
        rules={[{ required: true }, { min: 1, max: 127 }]}
      />
      <ProFormText
        name="zipCode"
        label={
          <FormattedMessage id="components.postal.address-form.fields.zip-code.label" />
        }
        rules={[{ required: true }, { min: 1, max: 15 }]}
      />
      <ProFormText
        name="passcode"
        label={
          <FormattedMessage id="components.postal.address-form.fields.passcode.label" />
        }
        rules={[{ min: 1, max: 15 }]}
      />
      <ProFormText
        name="googleMap"
        label={
          <FormattedMessage id="components.postal.address-form.fields.google-map.label" />
        }
        rules={[{ min: URL_MIN_LENGTH, max: URL_MAX_LENGTH }]}
      />
      <ProFormText
        name="aMap"
        label="高德地图"
        rules={[{ min: URL_MIN_LENGTH, max: URL_MAX_LENGTH }]}
      />
    </ModalForm>
  );
};

export default Widget;
