import { useAppSelector } from "../hooks";
import { selectLayout } from "../reducers/layout";

interface IProps {
  value: string;
}

const Widget = ({ value }: IProps) => {
  const site = useAppSelector(selectLayout);
  const t = `${value}|${site?.subhead || ""}|${site?.title || ""}`;
  return (
    <>
      <title>{t}</title>
      {value}
    </>
  );
};

export default Widget;
