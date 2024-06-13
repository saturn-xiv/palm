import { useEffect } from "react";
import { Col } from "antd";

import { useAppDispatch, useAppSelector } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import { PERSONAL_PROFILE_PATH } from "../../../Router";
import ChangePassword from "./ChangePassword";
import UpdateForm from "./UpdateForm";
import CancelAccount from "./CancelAccount";
import { currentUser as selectCurrentUser } from "../../../reducers/current-user";
import { is_email_user } from "../../../api/users";

export const Component = () => {
  const dispatch = useAppDispatch();
  const current_user = useAppSelector(selectCurrentUser);
  useEffect(() => {
    dispatch(set_pathname(PERSONAL_PROFILE_PATH));
  }, [dispatch]);
  return (
    <>
      <Col sm={24} md={{ span: 10 }}>
        <UpdateForm />
      </Col>
      {is_email_user(current_user.providerType) && (
        <Col sm={24} md={{ span: 10 }}>
          <ChangePassword />
        </Col>
      )}
      <Col sm={24} md={{ span: 10 }}>
        <CancelAccount />
      </Col>
    </>
  );
};
