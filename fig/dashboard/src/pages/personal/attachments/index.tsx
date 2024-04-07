import { useEffect } from "react";

import { useAppDispatch } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import { PERSONAL_ATTACHMENTS_PATH } from "../../../Router";

export const Component = () => {
  const dispatch = useAppDispatch();
  useEffect(() => {
    dispatch(set_pathname(PERSONAL_ATTACHMENTS_PATH));
  }, [dispatch]);
  return <>atta</>;
};
