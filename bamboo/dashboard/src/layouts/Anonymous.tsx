import { FormattedMessage } from "react-intl";
import { Outlet } from "react-router";
import { NavLink } from "react-router";

const Widget = () => {
  return (
    <div className="grid">
      <div className="cell">
        <NavLink to="/anonymous/administrator/sign-in">
          <FormattedMessage id="pages.administrator.sign-in.title" />
        </NavLink>
      </div>
      <div className="cell">
        <Outlet />
      </div>
    </div>
  );
};

export default Widget;
