import { IAttachment } from "../../api/daffodil";
import cover_img from "../../assets/calculator.svg";

interface IProps {
  items: IAttachment[];
}

const Widget = ({ items }: IProps) => {
  for (const it of items) {
    if (it.contentType.startsWith("image/")) {
      return <img alt={it.title} src={it.url} />;
    }
  }
  return <img alt="cover" src={cover_img} />;
};

export default Widget;
