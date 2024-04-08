import { Outlet } from "react-router-dom";
import { Row } from "antd";

export const Component = () => {
  return (
    <Row justify="center" gutter={16}>
      <Outlet />
    </Row>
  );
};
