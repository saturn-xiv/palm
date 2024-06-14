import { useState } from "react";
import { UploadOutlined } from "@ant-design/icons";
import { Upload, Button, message } from "antd";
import { useIntl } from "react-intl";

import { upload_url as upload_attachment_url } from "../../../api/attachments";

interface IProps {
  resource?: IResource;
}
interface IResource {
  type: string;
  id: number;
}

interface IS3File {
  bucket: string;
  object: string;
}

const Widget = ({ resource }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const [files, setFiles] = useState<Map<string, IS3File>>(
    new Map<string, IS3File>()
  );
  const intl = useIntl();

  return (
    <Upload
      multiple
      name="file"
      action={async (file) => {
        const it = await upload_attachment_url(file.name, file.type, file.size);
        const tmp = new Map<string, IS3File>([
          [file.uid, { bucket: it.bucket, object: it.object }],
        ]);
        setFiles(new Map([...files.entries(), ...tmp.entries()]));
        return it.url;
      }}
      method="put"
      onChange={(info) => {
        if (info.file.status !== "uploading") {
          console.log(info.file, info.fileList);
        }
        if (info.file.status === "done") {
          messageApi.success(
            intl.formatMessage(
              { id: "personal.attachments.upload.succeed" },
              { title: info.file.name }
            )
          );
        } else if (info.file.status === "error") {
          messageApi.error(
            intl.formatMessage(
              { id: "personal.attachments.upload.failed" },
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
