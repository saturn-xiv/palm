import { lazy } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

const Dashboard = lazy(() => import("./layouts/dashboard"));
const Application = lazy(() => import("./layouts/application"));

const Home = lazy(() => import("./pages/home"));
const UsersSignIn = lazy(() => import("./pages/users/sign-in"));
const UsersSignUp = lazy(() => import("./pages/users/sign-up"));
const UsersRestPassword = lazy(() => import("./pages/users/reset-password"));
const UsersForgotPassword = lazy(() => import("./pages/users/forgot-password"));
const UsersUnlockByEmail = lazy(() => import("./pages/users/unlock/by-email"));
const UsersUnlockByToken = lazy(() => import("./pages/users/unlock/by-token"));
const UsersConfirmByEmail = lazy(
  () => import("./pages/users/confirm/by-email")
);
const UsersConfirmByToken = lazy(
  () => import("./pages/users/confirm/by-token")
);
const UsersPersonal = lazy(() => import("./pages/users/personal"));
const NewLeaveWords = lazy(() => import("./pages/leave-words/new"));
const AdminUsers = lazy(() => import("./pages/admin/users"));
const AdminLeaveWords = lazy(() => import("./pages/admin/leave-words"));
const AdminPolicies = lazy(() => import("./pages/admin/policies"));
const AdminLocales = lazy(() => import("./pages/admin/locales"));
const AdminSiteBase = lazy(() => import("./pages/admin/site/base"));
const AdminSiteChina = lazy(() => import("./pages/admin/site/china"));
const AdminSiteSeo = lazy(() => import("./pages/admin/site/seo"));
const AdminSiteSmtp = lazy(() => import("./pages/admin/site/smtp"));
const AdminSiteStatus = lazy(() => import("./pages/admin/site/status"));

const router = createBrowserRouter(
  [
    {
      path: "dashboard",
      element: <Dashboard />,
      children: [
        {
          path: "personal",
          element: <UsersPersonal />,
        },
        {
          path: "admin",
          children: [
            { path: "locales", element: <AdminLocales /> },
            { path: "users", element: <AdminUsers /> },
            { path: "policies", element: <AdminPolicies /> },
            { path: "leave-words", element: <AdminLeaveWords /> },
            { path: "site/status", element: <AdminSiteStatus /> },
            { path: "site/base", element: <AdminSiteBase /> },
            { path: "site/seo", element: <AdminSiteSeo /> },
            { path: "site/smtp", element: <AdminSiteSmtp /> },
            { path: "site/seo", element: <AdminSiteSeo /> },
            { path: "site/china", element: <AdminSiteChina /> },
          ],
        },
      ],
    },
    {
      path: "anonymous",
      element: <Application />,
      children: [
        { path: "users/sign-in", element: <UsersSignIn /> },
        { path: "users/sign-up", element: <UsersSignUp /> },
        { path: "users/forgot-password", element: <UsersForgotPassword /> },
        { path: "users/reset-password", element: <UsersRestPassword /> },
        { path: "users/confirm/by-email", element: <UsersConfirmByEmail /> },
        {
          path: "users/confirm/by-token/:token",
          element: <UsersConfirmByToken />,
        },
        { path: "users/unlock/by-email", element: <UsersUnlockByEmail /> },
        {
          path: "users/unlock/by-token/:token",
          element: <UsersUnlockByToken />,
        },
        { path: "leave-words/new", element: <NewLeaveWords /> },
      ],
    },
    {
      path: "/",
      element: <Home />,
    },
  ],
  { basename: import.meta.env.BASE_URL }
);

const Widget = () => {
  return <RouterProvider router={router} />;
};

export default Widget;
