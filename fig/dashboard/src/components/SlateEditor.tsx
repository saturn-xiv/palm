import { useState } from "react";
import { createEditor, Descendant } from "slate";
import { Slate, Editable, withReact } from "slate-react";
import { withHistory } from "slate-history";

export const EDITOR = "slate";

interface IProps {
  defaultValue: Descendant[];
  handleChange: (value: string) => void;
}

const Widget = ({ defaultValue, handleChange }: IProps) => {
  const [editor] = useState(() => withReact(withHistory(createEditor())));
  return (
    <Slate
      editor={editor}
      onChange={(value) => {
        const isAstChange = editor.operations.some(
          (op) => "set_selection" !== op.type
        );
        if (isAstChange) {
          const content = JSON.stringify(value);
          handleChange(content);
        }
      }}
      initialValue={defaultValue}
    >
      <Editable spellCheck />
    </Slate>
  );
};

export default Widget;
