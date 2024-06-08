defmodule TuberoseWeb.Router do
  require Logger
  use TuberoseWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug(:accepts, ["html"])
    plug(:fetch_session)
    plug(:fetch_live_flash)
    plug(:put_root_layout, html: {TuberoseWeb.Layouts, :root})
    plug(:protect_from_forgery)
    plug(:put_secure_browser_headers)
  end

  pipeline :api do
    plug(:accepts, ["json"])
  end

  pipeline :admins_only do
    plug(:admin_basic_auth)
  end

  scope "/", TuberoseWeb do
    pipe_through(:browser)

    get("/", PageController, :home)
  end

  scope "/etc", TuberoseWeb.Etc do
    pipe_through(:browser)

    get("/nginx.conf", NginxConfController, :get)
    get("/systemd.conf", SystemdConfController, :get)
  end

  scope "/api/users", TuberoseWeb.Api do
    pipe_through(:api)

    post("/sign-in", UsersController, :sign_in)
    post("/sign-up", UsersController, :sign_up)
    post("/confirm", UsersController, :confirm_by_email)
    post("/confirm/:token", UsersController, :confirm_by_token)
    post("/unlock", UsersController, :unlock_by_email)
    post("/unlock/:token", UsersController, :unlock_by_token)
    post("/forgot-password", UsersController, :forgot_password)
    post("/reset-password/:token", UsersController, :reset_password)

    get("/profile", UsersController, :get_profile)
    post("/profile", UsersController, :set_profile)
    get("/logs", UsersController, :logs)
    post("/change-password", UsersController, :change_password)
    delete("/sign-out", UsersController, :sign_out)
  end

  scope "/api", TuberoseWeb.Api do
    pipe_through(:api)

    get("/layout", HomeController, :layout)

    resources("/locales", LocalesController, only: [:show, :index, :create, :update, :delete])

    resources("/leave-words", LeaveWordsController, only: [:show, :index, :create, :delete])
    post("/leave-words/:id/publish", LeaveWordsController, :publish)
    delete("/leave-words/:id/revoke", LeaveWordsController, :revoke)

    resources("/attachments", AttachmentsController,
      only: [:show, :index, :create, :update, :delete]
    )

    post("/attachments/:id/associate", AttachmentsController, :associate)
    post("/attachments/:id/dissociate", AttachmentsController, :dissociate)
  end

  scope "/dev" do
    pipe_through([:browser, :admins_only])

    live_dashboard("/dashboard", metrics: TuberoseWeb.Telemetry)
    forward("/mailbox", Plug.Swoosh.MailboxPreview)
  end

  defp admin_basic_auth(conn, _opts) do
    user = Application.get_env(:tuberose, TuberoseWeb.BasicAuthUser)
    Logger.info("$$$$$ #{inspect(user)}")
    Plug.BasicAuth.basic_auth(conn, username: user[:name], password: user[:password])
  end
end
