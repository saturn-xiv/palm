import { useState } from "react";
import { UploadOutlined } from "@ant-design/icons";
import { Upload, Button, message } from "antd";
import { useIntl } from "react-intl";

import {
  upload_url as upload_attachment_url,
  set_uploaded as set_attachment_uploaded,
  attach as attach_attachment,
} from "../../../api/attachments";

interface IProps {
  resource?: IResource;
  handleRefresh: () => void;
}
interface IResource {
  type: string;
  id: number;
}

interface IS3File {
  id: number;
  bucket: string;
  object: string;
}

const Widget = ({ resource, handleRefresh }: IProps) => {
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
          [file.uid, { id: it.id, bucket: it.bucket, object: it.object }],
        ]);
        setFiles((items) => new Map([...items.entries(), ...tmp.entries()]));
        return it.url;
      }}
      method="put"
      onChange={async (info) => {
        const it = files.get(info.file.uid);
        if (it) {
          if (info.file.status === "done") {
            await set_attachment_uploaded(it.id, true);
            if (resource) {
              await attach_attachment(it.id, resource.type, resource.id);
            }
            messageApi.info({
              type: "success",
              content: intl.formatMessage(
                { id: "personal.attachments.upload.succeed" },
                { title: info.file.name }
              ),
              onClose: handleRefresh,
              duration: 2,
            });
          } else if (info.file.status === "error") {
            await set_attachment_uploaded(it.id, false);
            messageApi.error(
              intl.formatMessage(
                { id: "personal.attachments.upload.failed" },
                { title: info.file.name }
              )
            );
          }
        }
      }}
    >
      {contextHolder}
      <Button icon={<UploadOutlined />} onClick={() => {}} />
    </Upload>
  );
};

export default Widget;
