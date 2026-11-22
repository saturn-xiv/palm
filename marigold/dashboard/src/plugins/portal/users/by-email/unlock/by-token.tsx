import { useParams } from "react-router";

const Widget = () => {
  const { token } = useParams();
  // TODO
  return <>Unlock by token: {token}</>;
};

export default Widget;
