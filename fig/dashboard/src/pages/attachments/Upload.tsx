import { InboxOutlined } from "@ant-design/icons";
import { message, Upload } from "antd";
import { FormattedMessage, useIntl } from "react-intl";

import { IResource, get as get_token } from "../../reducers/current-user";
import { IAttachment } from "../../api/daffodil";

const { Dragger } = Upload;

interface IProps {
  resource: IResource;
}

const Widget = ({ resource }: IProps) => {
  const [messageApi, contextHolder] = message.useMessage();
  const intl = useIntl();
  return (
    <>
      {contextHolder}
      <Dragger
        name="file"
        multiple
        action="/api/attachments/"
        // https://github.com/react-component/upload#customrequest
        customRequest={(options) => {
          const data = new FormData();
          data.append("file", options.file);
          data.append(
            "json",
            new Blob(
              [
                JSON.stringify({
                  resource,
                }),
              ],
              {
                type: "application/json",
              }
            )
          );
          fetch(options.action, {
            method: "POST",
            body: data,
            headers: {
              Authorization: `Bearer ${get_token()}`,
            },
          })
            .then((res: IAttachment) => {
              console.log(res);
              options.onSuccess(res.object, options.file);
            })
            .catch((e) => {
              messageApi.error(e);
            });
        }}
        onChange={(info) => {
          const { status } = info.file;
          if (status === "done") {
            messageApi.success(
              intl.formatMessage(
                { id: "pages.attachments.upload.done" },
                { name: info.file.name }
              )
            );
          } else if (status === "error") {
            messageApi.error(
              intl.formatMessage(
                { id: "pages.attachments.upload.failed" },
                { name: info.file.name }
              )
            );
          }
        }}
      >
        <p className="ant-upload-drag-icon">
          <InboxOutlined />
        </p>
        <p className="ant-upload-text">
          <FormattedMessage id="pages.attachments.upload.help" />
        </p>
        <p className="ant-upload-hint">
          <FormattedMessage id="pages.attachments.upload.hint" />
        </p>
      </Dragger>
    </>
  );
};

export default Widget;
