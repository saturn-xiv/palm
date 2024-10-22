import { Outlet } from "react-router-dom";

const Widget = () => {
  return (
    <div>
      <div>application header</div>
      <div>
        <Outlet />
      </div>
      <div>application footer</div>
    </div>
  );
};

export default Widget;
