import { useAppSelector } from "../hooks";
import { selectLayout as selectSiteLayout } from "../reducers/layout";

interface IProps {
  value: string;
}

const Widget = ({ value }: IProps) => {
  const site = useAppSelector(selectSiteLayout);
  return (
    <>
      <title>
        {value}|{site?.subhead || ""}|{site?.title || ""}
      </title>
      <h2>{value}</h2>
    </>
  );
};

export default Widget;
