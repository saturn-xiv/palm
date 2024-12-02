import { Typography } from "antd";
import { useState } from "react";

interface IProps {
  text: string;
  copyable?: boolean;
}

const Widget = ({ text, copyable }: IProps) => {
  const [expanded, setExpanded] = useState(false);
  return (
    <Typography.Paragraph
      ellipsis={{
        rows: 2,
        expandable: "collapsible",
        expanded,
        onExpand: (_, info) => setExpanded(info.expanded),
      }}
      copyable={copyable}
    >
      {text}
    </Typography.Paragraph>
  );
};

export default Widget;
