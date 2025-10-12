import { Outlet } from "react-router";

const Widget = () => {
  return (
    <>
      <h1>root layout</h1>
      <Outlet />
    </>
  );
};

export default Widget;
