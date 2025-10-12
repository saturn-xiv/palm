import { Outlet } from "react-router";

const Widget = () => {
  return (
    <>
      <h1>dashboard layout</h1>
      <Outlet />
    </>
  );
};

export default Widget;
