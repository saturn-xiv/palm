import { Outlet } from "react-router";

import Footer from "../Footer";

const Widget = () => {
  // TODO
  return (
    <div>
      <div>dashboard layout</div>
      <Outlet />
      <div>
        <Footer />
      </div>
    </div>
  );
};

export default Widget;
