import { useEffect } from "react";
import { Col } from "antd";

import { useAppDispatch } from "../../../hooks";
import { set_pathname } from "../../../reducers/side-bar";
import { PERSONAL_ATTACHMENTS_PATH } from "../../../Router";
import Table from "./PaginationTable";

export const Component = () => {
  const dispatch = useAppDispatch();
  useEffect(() => {
    dispatch(set_pathname(PERSONAL_ATTACHMENTS_PATH));
  }, [dispatch]);
  return (
    <Col sm={24} md={{ span: 22 }}>
      <Table />
    </Col>
  );
};
