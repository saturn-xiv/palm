import { Outlet } from "react-router-dom";

import Copyright from "../Copyright";

const Widget = () => {
  return (
    <div>
      <div>application header</div>
      <div>
        <Outlet />
      </div>
      <div>
        application footer
        <Copyright />
      </div>
    </div>
  );
};

export default Widget;
