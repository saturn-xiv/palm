import { useParams } from "react-router-dom";

export function Component() {
  const { token } = useParams();
  return <>unlock by token: {token}</>;
}
