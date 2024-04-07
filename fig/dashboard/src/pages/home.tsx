import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { get as get_token } from "../reducers/current-user";
import { SELF_PATH, USERS_SIGN_IN_PATH } from "../Router";

export function Component() {
  const navigate = useNavigate();
  const token = get_token();
  useEffect(() => {
    navigate(token === null ? USERS_SIGN_IN_PATH : SELF_PATH);
  }, [navigate, token]);
  return <></>;
}
