import { Link, Outlet } from "react-router";
import { FormattedMessage } from "react-intl";

import logo_svg from "../assets/bamboo.svg";

const NavBar = () => {
  const handleSignOut = () => {
    // TODO
    console.log("sign out");
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
        <a className="navbar-item">
          <img src={logo_svg} />
        </a>

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
          <Link className="navbar-item" to="/dashboard/users">
            <FormattedMessage id="pages.users.index.title" />
          </Link>
          <Link className="navbar-item" to="/dashboard/rules">
            <FormattedMessage id="pages.rules.index.title" />
          </Link>

          <div className="navbar-item has-dropdown is-hoverable">
            <a className="navbar-link">
              <FormattedMessage id="layouts.dashboard.nav-bar.account" />
            </a>

            <div className="navbar-dropdown">
              <Link className="navbar-item" to="/dashboard/administrator/logs">
                <FormattedMessage id="pages.administrator.logs.title" />
              </Link>
              <Link
                className="navbar-item"
                to="/dashboard/administrator/profile"
              >
                <FormattedMessage id="pages.administrator.profile.title" />
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
