import { createBrowserRouter, RouterProvider } from "react-router-dom";

const router = createBrowserRouter(
  [
    { path: "/", lazy: () => import("./pages/home") },
    {
      path: "/dashboard",
      lazy: () => import("./layouts/dashboard"),
      children: [
        { path: "main", lazy: () => import("./pages/main") },
        { path: "settings", lazy: () => import("./pages/settings") },
        { path: "self", lazy: () => import("./pages/self") },
        { path: "attachments", lazy: () => import("./pages/attachments") },
        {
          path: "daffodil/ledgers",
          lazy: () => import("./pages/daffodil/ledgers"),
        },
        {
          path: "daffodil/ledgers/new",
          lazy: () => import("./pages/daffodil/ledgers/new"),
        },
        {
          path: "daffodil/ledgers/:id",
          lazy: () => import("./pages/daffodil/ledgers/:id/show"),
        },
        {
          path: "daffodil/ledgers/:id/edit",
          lazy: () => import("./pages/daffodil/ledgers/:id/edit"),
        },
        {
          path: "daffodil/ledgers/:id/append-bill",
          lazy: () => import("./pages/daffodil/ledgers/:id/append-bill"),
        },
        {
          path: "daffodil/bills/:id",
          lazy: () => import("./pages/daffodil/bills/:id/show"),
        },
        {
          path: "daffodil/bills/:id/edit",
          lazy: () => import("./pages/daffodil/bills/:id/edit"),
        },
      ],
    },
    {
      path: "/anonymous",
      lazy: () => import("./layouts/application"),
      children: [
        { path: "users/sign-in", lazy: () => import("./pages/users/sign-in") },
        { path: "users/sign-up", lazy: () => import("./pages/users/sign-up") },
        {
          path: "users/forgot-password",
          lazy: () => import("./pages/users/forgot-password"),
        },
        {
          path: "users/reset-password/:token",
          lazy: () => import("./pages/users/reset-password"),
        },
        {
          path: "users/confirm",
          lazy: () => import("./pages/users/confirm/by-email"),
        },
        {
          path: "users/confirm/:token",
          lazy: () => import("./pages/users/confirm/by-token"),
        },
        {
          path: "users/unlock",
          lazy: () => import("./pages/users/unlock/by-email"),
        },
        {
          path: "users/unlock/:token",
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
