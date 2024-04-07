import { useEffect } from "react";

import { useAppDispatch } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import { PERSONAL_PROFILE_PATH } from "../../../Router";

export const Component = () => {
  const dispatch = useAppDispatch();
  useEffect(() => {
    dispatch(set_pathname(PERSONAL_PROFILE_PATH));
  }, [dispatch]);
  return <>profile</>;
};
