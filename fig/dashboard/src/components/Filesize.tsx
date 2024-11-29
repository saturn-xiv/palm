import { filesize } from "filesize";

interface IProps {
  value: number;
}

const Widget = ({ value }: IProps) => {
  return <>{filesize(value, { standard: "jedec" })}</>;
};

export default Widget;
