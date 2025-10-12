import { lazy } from "react";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";

const AdministratorSignIn = lazy(() => import("./pages/administrator/sign-in"));
const AdministratorLogs = lazy(() => import("./pages/administrator/logs"));
const AdministratorProfile = lazy(
  () => import("./pages/administrator/profile")
);
const UsersIndex = lazy(() => import("./pages/users"));
const RulesIndex = lazy(() => import("./pages/rules"));
const HostsIndex = lazy(() => import("./pages/hosts"));
const DashboardIndex = lazy(() => import("./pages/dashboard"));
const RootLayout = lazy(() => import("./layouts/Root"));
const AnonymousLayout = lazy(() => import("./layouts/Anonymous"));
const DashboardLayout = lazy(() => import("./layouts/Dashboard"));
const Home = lazy(() => import("./pages/home"));

const router = createBrowserRouter([
  {
    path: "/",
    Component: RootLayout,
    children: [
      { index: true, Component: Home },
      {
        path: "anonymous",
        Component: AnonymousLayout,
        children: [
          {
            path: "administrator",
            children: [{ path: "sign-in", Component: AdministratorSignIn }],
          },
        ],
      },
      {
        path: "dashboard",
        Component: DashboardLayout,
        children: [
          { index: true, Component: DashboardIndex },
          { path: "hosts", Component: HostsIndex },
          { path: "users", Component: UsersIndex },
          { path: "rules", Component: RulesIndex },
          {
            path: "administrator",
            children: [
              {
                path: "profile",
                Component: AdministratorProfile,
              },
              { path: "logs", Component: AdministratorLogs },
            ],
          },
        ],
      },
    ],
  },
]);

const Widget = () => {
  return <RouterProvider router={router} />;
};

export default Widget;
