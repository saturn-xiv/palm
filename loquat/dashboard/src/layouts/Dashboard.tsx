import { Link, Outlet, useNavigate } from "react-router";
import { FormattedMessage, useIntl } from "react-intl";

import {
  SIGN_IN as USER_SIGN_IN,
  LOGS as USER_LOGS,
  signOut,
} from "../reducers/session";
import { useAppDispatch } from "../hooks";
import { success as show_success } from "../reducers/notification";

const NavBar = () => {
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const intl = useIntl();
  const handleSignOut = () => {
    dispatch(signOut());
    navigate(USER_SIGN_IN);
    dispatch(show_success([intl.formatMessage({ id: "flashes.succeed" })]));
  };
  const handleApply = () => {
    // TODO
    console.log("apply");
  };
  const handleReboot = () => {
    // TODO
    console.log("reboot");
  };
  return (
    <nav className="navbar" role="navigation" aria-label="main navigation">
      <div className="navbar-brand">
        <a className="navbar-item"></a>

        <a
          role="button"
          className="navbar-burger"
          aria-label="menu"
          aria-expanded="false"
          data-target="navbarDashboardLayout"
        >
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>

      <div id="navbarDashboardLayout" className="navbar-menu">
        <div className="navbar-start">
          <Link className="navbar-item" to="/dashboard">
            <FormattedMessage id="layouts.dashboard.nav-bar.home" />
          </Link>
          <Link className="navbar-item" to="/dashboard/hosts">
            <FormattedMessage id="pages.hosts.index.title" />
          </Link>
          <Link className="navbar-item" to="/dashboard/members">
            <FormattedMessage id="pages.members.index.title" />
          </Link>
          <Link className="navbar-item" to="/dashboard/rules">
            <FormattedMessage id="pages.rules.index.title" />
          </Link>

          <div className="navbar-item has-dropdown is-hoverable">
            <a className="navbar-link">
              <FormattedMessage id="layouts.dashboard.nav-bar.account" />
            </a>

            <div className="navbar-dropdown">
              <Link className="navbar-item" to={USER_LOGS}>
                <FormattedMessage id="pages.users.logs.title" />
              </Link>
              <Link
                className="navbar-item"
                to="/dashboard/account/change-password"
              >
                <FormattedMessage id="pages.users.change-password.title" />
              </Link>
              <hr className="navbar-divider" />
              <a className="navbar-item" onClick={handleSignOut}>
                <FormattedMessage id="layouts.dashboard.nav-bar.sign-out" />
              </a>
            </div>
          </div>
        </div>

        <div className="navbar-end">
          <div className="navbar-item">
            <div className="buttons">
              <a
                className="button is-info is-small"
                onClick={handleApply}
                href="#"
              >
                <strong>
                  <FormattedMessage id="layouts.dashboard.nav-bar.apply" />
                </strong>
              </a>
              <a
                className="button is-warning is-small"
                onClick={handleReboot}
                href="#"
              >
                <strong>
                  <FormattedMessage id="layouts.dashboard.nav-bar.reboot" />
                </strong>
              </a>
              <a
                className="button is-light is-small"
                onClick={handleSignOut}
                href="#"
              >
                <FormattedMessage id="layouts.dashboard.nav-bar.sign-out" />
              </a>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
};

const Widget = () => {
  return (
    <div className="fixed-grid has-4-cols">
      <div className="grid">
        <div className="cell is-col-span-4">
          <NavBar />
          <Outlet />
        </div>
      </div>
    </div>
  );
};

export default Widget;
