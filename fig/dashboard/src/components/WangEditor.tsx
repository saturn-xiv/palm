import { useEffect, useState } from "react";
import { Editor, Toolbar } from "@wangeditor/editor-for-react";
import { IDomEditor, IEditorConfig, IToolbarConfig } from "@wangeditor/editor";
import { useIntl } from "react-intl";

import "@wangeditor/editor/dist/css/style.css";

export const EDITOR = "wang";

interface IProps {
  html: string;
  handleChange: (value: string) => void;
}

const Widget = ({ html, handleChange }: IProps) => {
  const intl = useIntl();
  const [editor, setEditor] = useState<IDomEditor | null>(null);

  const toolbarConfig: Partial<IToolbarConfig> = {};
  const editorConfig: Partial<IEditorConfig> = {
    placeholder: intl.formatMessage({ id: "form.placeholder.type-here" }),
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
  return (
    <>
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
      <div style={{ marginTop: "15px" }}>{html}</div>
    </>
  );
};

export default Widget;
