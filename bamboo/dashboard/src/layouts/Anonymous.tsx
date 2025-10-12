import { Outlet } from "react-router";

const Widget = () => {
  return (
    <>
      <h1>anonymous layout</h1>
      <Outlet />
    </>
  );
};

export default Widget;
