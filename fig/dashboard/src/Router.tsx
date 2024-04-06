import { createBrowserRouter, RouterProvider } from "react-router-dom";

export const USERS_SIGN_UP_PATH = "/anonymous/users/sign-up";
export const USERS_SIGN_IN_PATH = "/anonymous/users/sign-in";
export const USERS_CONFIRM_BY_EMAIL_PATH = "/anonymous/users/confirm";
export const USERS_CONFIRM_BY_TOKEN_PATH = "/anonymous/users/confirm/:token";
export const USERS_UNLOCK_BY_EMAIL_PATH = "/anonymous/users/unlock";
export const USERS_UNLOCK_BY_TOKEN_PATH = "/anonymous/users/unlock/:token";
export const USERS_FORGOT_PASSWORD_PATH = "/anonymous/users/forgot-password";
export const USERS_RESET_PASSWORD_PATH =
  "/anonymous/users/reset-password/:token";
export const LEAVE_WORDS_NEW_PATH = "/anonymous/leave-words/new";

export const SELF_PATH = "/dashboard/self";

const router = createBrowserRouter(
  [
    { path: "/", lazy: () => import("./pages/home") },
    {
      path: "/dashboard",
      lazy: () => import("./layouts/dashboard"),
      children: [
        // { path: "main", lazy: () => import("./pages/main") },
        // { path: "settings", lazy: () => import("./pages/settings") },
        { path: "self", lazy: () => import("./pages/self") },
        // { path: "attachments", lazy: () => import("./pages/attachments") },
        // {
        //   path: "daffodil/ledgers",
        //   lazy: () => import("./pages/daffodil/ledgers"),
        // },
        // {
        //   path: "daffodil/ledgers/new",
        //   lazy: () => import("./pages/daffodil/ledgers/new"),
        // },
        // {
        //   path: "daffodil/ledgers/:id",
        //   lazy: () => import("./pages/daffodil/ledgers/:id/show"),
        // },
        // {
        //   path: "daffodil/ledgers/:id/edit",
        //   lazy: () => import("./pages/daffodil/ledgers/:id/edit"),
        // },
        // {
        //   path: "daffodil/ledgers/:id/append-bill",
        //   lazy: () => import("./pages/daffodil/ledgers/:id/append-bill"),
        // },
        // {
        //   path: "daffodil/bills/:id",
        //   lazy: () => import("./pages/daffodil/bills/:id/show"),
        // },
        // {
        //   path: "daffodil/bills/:id/edit",
        //   lazy: () => import("./pages/daffodil/bills/:id/edit"),
        // },
      ],
    },
    {
      path: "/anonymous",
      lazy: () => import("./layouts/application"),
      children: [
        {
          path: USERS_SIGN_IN_PATH,
          lazy: () => import("./pages/users/sign-in"),
        },
        {
          path: USERS_SIGN_UP_PATH,
          lazy: () => import("./pages/users/sign-up"),
        },
        {
          path: USERS_FORGOT_PASSWORD_PATH,
          lazy: () => import("./pages/users/forgot-password"),
        },
        {
          path: USERS_RESET_PASSWORD_PATH,
          lazy: () => import("./pages/users/reset-password"),
        },
        {
          path: USERS_CONFIRM_BY_EMAIL_PATH,
          lazy: () => import("./pages/users/confirm/by-email"),
        },
        {
          path: USERS_CONFIRM_BY_TOKEN_PATH,
          lazy: () => import("./pages/users/confirm/by-token"),
        },
        {
          path: USERS_UNLOCK_BY_EMAIL_PATH,
          lazy: () => import("./pages/users/unlock/by-email"),
        },
        {
          path: USERS_UNLOCK_BY_TOKEN_PATH,
          lazy: () => import("./pages/users/unlock/by-token"),
        },
        // {
        //   path: "leave-words/new",
        //   lazy: () => import("./pages/leave-words/new"),
        // },
      ],
    },
    { path: "*", lazy: () => import("./pages/not-found") },
  ],
  {
    basename: import.meta.env.BASE_URL,
  }
);

const Widget = () => <RouterProvider router={router} />;

export default Widget;
