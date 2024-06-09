import { createBrowserRouter, RouterProvider } from "react-router-dom";

export const USERS_SIGN_UP_PATH = "/anonymous/users/sign-up";
export const USERS_SIGN_IN_PATH = "/anonymous/users/sign-in";
export const USERS_CONFIRM_BY_EMAIL_PATH = "/anonymous/users/confirm-by-email";
export const USERS_CONFIRM_BY_TOKEN_PATH = "/anonymous/users/confirm/:token";
export const USERS_UNLOCK_BY_EMAIL_PATH = "/anonymous/users/unlock-by-email";
export const USERS_UNLOCK_BY_TOKEN_PATH = "/anonymous/users/unlock/:token";
export const USERS_FORGOT_PASSWORD_BY_EMAIL_PATH =
  "/anonymous/users/forgot-password-by-email";
export const USERS_RESET_PASSWORD_PATH =
  "/anonymous/users/reset-password/:token";
export const LEAVE_WORDS_NEW_PATH = "/anonymous/leave-words/new";

export const PERSONAL_LOGS_PATH = "/dashboard/personal/logs";
export const PERSONAL_PROFILE_PATH = "/dashboard/personal/profile";
export const PERSONAL_ATTACHMENTS_PATH = "/dashboard/personal/attachments";

export const SETTINGS_SITE_SEO_PATH = "/dashboard/settings/site/seo";
export const SETTINGS_SITE_INFO_PATH = "/dashboard/settings/site/info";
export const SETTINGS_SITE_STATUS_PATH = "/dashboard/settings/site/status";
export const SETTINGS_LOCALES_PATH = "/dashboard/settings/locales";

const router = createBrowserRouter(
  [
    { path: "/", lazy: () => import("./pages/home") },
    {
      path: "dashboard",
      lazy: () => import("./layouts/dashboard"),
      children: [
        {
          path: "personal",
          lazy: () => import("./pages/personal"),
          children: [
            { path: "logs", lazy: () => import("./pages/personal/logs") },
            { path: "profile", lazy: () => import("./pages/personal/profile") },
            {
              path: "attachments",
              lazy: () => import("./pages/personal/attachments"),
            },
          ],
        },
        {
          path: "settings",
          lazy: () => import("./pages/settings"),
          children: [
            {
              path: "site/seo",
              lazy: () => import("./pages/settings/site/seo"),
            },
            {
              path: "site/info",
              lazy: () => import("./pages/settings/site/info"),
            },
            {
              path: "site/status",
              lazy: () => import("./pages/settings/site/status"),
            },
            {
              path: "locales",
              lazy: () => import("./pages/settings/locales"),
            },
          ],
        },
        {
          path: "daffodil",
          lazy: () => import("./pages/daffodil"),
          children: [
            {
              path: "ledgers",
              lazy: () => import("./pages/daffodil/ledgers"),
            },
            {
              path: "ledgers/:id",
              lazy: () => import("./pages/daffodil/ledgers/:id/show"),
            },
          ],
        },

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
          path: USERS_FORGOT_PASSWORD_BY_EMAIL_PATH,
          lazy: () => import("./pages/users/forgot-password/by-email"),
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
        {
          path: "leave-words/new",
          lazy: () => import("./pages/leave-words/new"),
        },
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
