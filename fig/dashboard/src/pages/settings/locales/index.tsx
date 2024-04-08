import { useEffect } from "react";
import { Col } from "antd";

import { useAppDispatch } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import { SETTINGS_LOCALES_PATH } from "../../../Router";
import Table from "./Table";

export const Component = () => {
  const dispatch = useAppDispatch();
  useEffect(() => {
    dispatch(set_pathname(SETTINGS_LOCALES_PATH));
  }, [dispatch]);
  return (
    <Col sm={24} md={{ span: 22 }}>
      <Table />
    </Col>
  );
};
