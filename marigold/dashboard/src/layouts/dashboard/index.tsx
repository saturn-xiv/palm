import { Outlet } from "react-router";

import Footer from "../Footer";

// https://github.com/mui/material-ui/tree/v9.5.0/docs/data/material/getting-started/templates/dashboard
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
