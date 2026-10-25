import { useEffect } from "react";
import { useNavigate } from "react-router";

import { useAppSelector } from "../../hooks";
import { selectCurrentUser } from "../../reducers/current-user";
import { selectLayout } from "../../reducers/layout";
import {
  USERS_SIGN_IN_PATH,
  PERSONAL_LOGS_PATH,
} from "../../reducers/current-user";

const Widget = () => {
  const current_user = useAppSelector(selectCurrentUser);
  const site = useAppSelector(selectLayout);
  const navigate = useNavigate();
  useEffect(() => {
    if (current_user) {
      navigate(PERSONAL_LOGS_PATH);
    } else {
      navigate(USERS_SIGN_IN_PATH);
    }
  }, [current_user, site, navigate]);

  return <></>;
};

export default Widget;
