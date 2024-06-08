import { UploadOutlined } from "@ant-design/icons";
import { Upload, Button, message } from "antd";
import { useIntl } from "react-intl";

import { get as getToken } from "../../../reducers/current-user";

interface IProps {
  resource?: IResource;
}
interface IResource {
  type: string;
  id: number;
}

const Widget = ({ resource }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  return (
    <Upload
      multiple
      name="file"
      action={`/api/attachments${
        resource ? `?type=${resource.type}&id=${resource.id}` : ""
      }`}
      headers={{
        Authorization: `Bearer ${getToken()}`,
      }}
      onChange={(info) => {
        if (info.file.status !== "uploading") {
          console.log(info.file, info.fileList);
        }
        if (info.file.status === "done") {
          messageApi.success(
            intl.formatMessage(
              { id: "attachments.upload.succeed" },
              { title: info.file.name }
            )
          );
        } else if (info.file.status === "error") {
          messageApi.error(
            intl.formatMessage(
              { id: "attachments.upload.failed" },
              { title: info.file.name }
            )
          );
        }
      }}
    >
      {contextHolder}
      <Button icon={<UploadOutlined />} onClick={() => {}} />
    </Upload>
  );
};

export default Widget;
