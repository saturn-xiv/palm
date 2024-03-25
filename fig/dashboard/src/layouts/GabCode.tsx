import { IGabCode } from "../api/camelia";
import icon_img from "../assets/beian/1.jpg";

interface IProps {
  value: IGabCode;
}

const Widget = ({ value }: IProps) => {
  return (
    <>
      <img src={icon_img} style={{ float: "left" }} />
      &nbsp;
      <a
        href={`https://beian.mps.gov.cn/#/query/webSearch?code=${value.code}`}
        rel="noreferrer"
        target="_blank"
      >
        {value.name}
      </a>
    </>
  );
};

export default Widget;
