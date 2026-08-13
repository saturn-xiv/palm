import { lazy } from "react";
import { type RouteObject } from "react-router";

export interface IRouter {
  anonymous: RouteObject[];
  dashboard: RouteObject[];
}

const router: IRouter = {
  anonymous: [
    {
      path: "users/sign-in",
      Component: lazy(() => import("./users/sign-in")),
    },
    {
      path: "users/sign-up",
      Component: lazy(() => import("./users/sign-up")),
    },
    {
      path: "users/by-email/confirm",
      Component: lazy(() => import("./users/by-email/confirm/request")),
    },
    {
      path: "users/by-email/confirm/:token",
      Component: lazy(() => import("./users/by-email/confirm/by-token")),
    },
    {
      path: "users/by-email/unlock",
      Component: lazy(() => import("./users/by-email/unlock/request")),
    },
    {
      path: "users/by-email/unlock/:token",
      Component: lazy(() => import("./users/by-email/unlock/by-token")),
    },
    {
      path: "users/by-email/forgot-password",
      Component: lazy(() => import("./users/by-email/forgot-password")),
    },
    {
      path: "users/by-email/reset-password/:token",
      Component: lazy(() => import("./users/by-email/reset-password")),
    },
  ],
  dashboard: [],
};

export default router;
