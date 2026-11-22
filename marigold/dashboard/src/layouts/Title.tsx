import { useAppSelector } from "../hooks";
import { selectLayout } from "../reducers/layout";

interface IProps {
  value: string;
}

const Widget = ({ value }: IProps) => {
  const site = useAppSelector(selectLayout);
  return (
    <>
      <title>
        {value}|{site?.subhead || ""}|{site?.title || ""}
      </title>
      {value}
    </>
  );
};

export default Widget;
