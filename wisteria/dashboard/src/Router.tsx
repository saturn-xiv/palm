import { lazy } from "react";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";

import AnonymousLayout from "./layouts/anonymous";
import DashboardLayout from "./layouts/dashboard";
import portal from "./plugins/portal";
import cms from "./plugins/cms";
import forum from "./plugins/forum";

const router = createBrowserRouter(
  [
    {
      path: "/",
      Component: lazy(() => import("./plugins/portal/home")),
    },
    {
      path: "/anonymous",
      Component: AnonymousLayout,
      children: [...portal.anonymous, ...cms.anonymous, ...forum.anonymous],
    },
    {
      path: "/dashboard",
      Component: DashboardLayout,
      children: [...portal.dashboard, ...cms.dashboard, ...forum.dashboard],
    },
  ],
  {
    basename: import.meta.env.BASE_URL,
  },
);

const Widget = () => <RouterProvider router={router} />;

export default Widget;
