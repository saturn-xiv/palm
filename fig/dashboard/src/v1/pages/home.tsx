import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

import {
  get as get_token,
  SIGN_IN_PATH,
  PERSONAL_PATH,
} from "../reducers/current-user";

export function Component() {
  const navigate = useNavigate();
  const token = get_token();
  useEffect(() => {
    navigate(token === null ? SIGN_IN_PATH : PERSONAL_PATH);
  }, [navigate, token]);
  return <></>;
}
