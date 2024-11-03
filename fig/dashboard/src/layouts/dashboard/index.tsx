import { Outlet } from "react-router-dom";

import Copyright from "../Copyright";

const Widget = () => {
  return (
    <div>
      <div>dashboard header</div>
      <div>
        <Outlet />
      </div>
      <div>
        dashboard footer
        <Copyright />
      </div>
    </div>
  );
};

export default Widget;
