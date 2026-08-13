import { useParams } from "react-router";

const Widget = () => {
  const {token} = useParams();
  // TODO
  return <>Reset Password: {token} </>;
};

export default Widget;
