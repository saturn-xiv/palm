import Link from "@mui/material/Link";

interface IProps {
  code: string;
}

const Widget = ({ code }: IProps) => {
  return (
    <Link href="https://beian.miit.gov.cn/" rel="noreferrer" target="_blank">
      {code}
    </Link>
  );
};

export default Widget;
