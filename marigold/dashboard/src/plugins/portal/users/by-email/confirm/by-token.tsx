import { useParams } from "react-router";

const Widget = () => {
  const { token } = useParams();
  // TODO
  return <>Confirm by token: {token}</>;
};

export default Widget;
