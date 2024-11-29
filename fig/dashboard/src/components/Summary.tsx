import { convert } from "html-to-text";

interface IProps {
  html: string;
  wordwrap: number;
}
const Widget = ({ html, wordwrap }: IProps) => {
  return <>{convert(html, { wordwrap })}</>;
};

export default Widget;
