import Link from "@mui/material/Link";

import { IGabCode } from "../api/camelia";
import icon_img from "../assets/beian/1.jpg";

interface IProps {
  value: IGabCode;
}

const Widget = ({ value }: IProps) => {
  return (
    <>
      <img src={icon_img} style={{ width: "12px" }} />
      <Link
        href={`https://beian.mps.gov.cn/#/query/webSearch?code=${value.code}`}
        rel="noreferrer"
        target="_blank"
      >
        {value.name}
      </Link>
    </>
  );
};

export default Widget;
