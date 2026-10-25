import { useAppSelector } from "../hooks";
import { selectLayout } from "../reducers/layout";

const Widget = () => {
  const site = useAppSelector(selectLayout);
  return <>{site?.copyright || ""}</>;
};

export default Widget;
