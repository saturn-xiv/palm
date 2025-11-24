import { useEffect } from "react";
import { useNavigate } from "react-router";

import { useAppSelector } from "../hooks";
import {
  SIGN_IN as USERS_SIGN_IN,
  LOGS as USER_LOGS,
  currentUser,
} from "../reducers/session";

const Widget = () => {
  const navigate = useNavigate();
  const user = useAppSelector(currentUser);
  useEffect(() => {
    navigate(user ? USER_LOGS : USERS_SIGN_IN);
  }, [user, navigate]);

  return <></>;
};

export default Widget;
