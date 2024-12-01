import { useEffect, useState } from "react";
import { Editor, Toolbar } from "@wangeditor/editor-for-react";
import { IDomEditor, IEditorConfig, IToolbarConfig } from "@wangeditor/editor";
import { useIntl } from "react-intl";
import type { MessageInstance } from "antd/es/message/interface";

import "@wangeditor/editor/dist/css/style.css";

export const EDITOR = "WANG";

interface IProps {
  html: string;
  handleChange: (value: string) => void;
  messageApi: MessageInstance;
}

const Widget = ({ html, handleChange, messageApi }: IProps) => {
  const intl = useIntl();
  const [editor, setEditor] = useState<IDomEditor | null>(null);

  const toolbarConfig: Partial<IToolbarConfig> = {};
  const editorConfig: Partial<IEditorConfig> = {
    placeholder: intl.formatMessage({ id: "form.placeholder.type-here" }),
    MENU_CONF: {
      uploadImage: {
        server: "/api/wang-editor/upload/image",
        fieldName: "file",
        // 5M
        maxFileSize: 5 * 1024 * 1024,
        maxNumberOfFiles: 10,
        allowedFileTypes: ["image/*"],
        withCredentials: true,
        // 20s
        timeout: 20 * 1000,
        onSuccess(file: File) {
          messageApi.success(
            intl.formatMessage(
              { id: "components.wang-editor.upload.succeed" },
              { name: file.name }
            )
          );
        },
        onError(file: File) {
          messageApi.success(
            intl.formatMessage(
              { id: "components.wang-editor.upload.succeed" },
              { name: file.name }
            )
          );
        },
      },
      uploadVideo: {
        fieldName: "file",
        // 5M
        maxFileSize: 100 * 1024 * 1024,
        maxNumberOfFiles: 3,
        allowedFileTypes: ["video/*"],
        withCredentials: true,
        // 20s
        timeout: 20 * 1000,
      },
      onSuccess(file: File) {
        messageApi.success(
          intl.formatMessage(
            { id: "components.wang-editor.upload.succeed" },
            { name: file.name }
          )
        );
      },
      onError(file: File) {
        messageApi.success(
          intl.formatMessage(
            { id: "components.wang-editor.upload.succeed" },
            { name: file.name }
          )
        );
      },
    },
  };

  useEffect(() => {
    return () => {
      if (editor == null) {
        return;
      }
      editor.destroy();
      setEditor(null);
    };
  }, [editor]);
  // <div style={{ marginTop: "15px" }}>{html}</div>
  return (
    <div style={{ border: "1px solid #ccc", zIndex: 100 }}>
      <Toolbar
        editor={editor}
        defaultConfig={toolbarConfig}
        mode="default"
        style={{ borderBottom: "1px solid #ccc" }}
      />
      <Editor
        defaultConfig={editorConfig}
        value={html}
        onCreated={setEditor}
        onChange={(editor) => handleChange(editor.getHtml())}
        mode="default"
        style={{ height: "500px", overflowY: "hidden" }}
      />
    </div>
  );
};

export default Widget;
