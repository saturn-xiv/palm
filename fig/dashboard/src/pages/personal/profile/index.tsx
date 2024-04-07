import { useEffect } from "react";
import { Col } from "antd";

import { useAppDispatch } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import { PERSONAL_PROFILE_PATH } from "../../../Router";
import ChangePassword from "./ChangePassword";
import UpdateForm from "./UpdateForm";

export const Component = () => {
  const dispatch = useAppDispatch();
  useEffect(() => {
    dispatch(set_pathname(PERSONAL_PROFILE_PATH));
  }, [dispatch]);
  return (
    <>
      <Col sm={24} md={{ span: 10 }}>
        <UpdateForm />
      </Col>
      <Col sm={24} md={{ span: 10 }}>
        <ChangePassword />
      </Col>
    </>
  );
};
