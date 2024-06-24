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

  pipeline :graphql do
    plug(TuberoseWeb.Context)
  end

  scope "/", TuberoseWeb do
    pipe_through(:browser)

    get("/", PageController, :home)

    scope "/:lang" do
      get("/home.html", PageController, :home_by_lang)
    end

    scope "/etc", Etc do
      get("/nginx.conf", NginxConfController, :get)
      get("/systemd.conf", SystemdConfController, :get)
    end
  end

  scope "/" do
    pipe_through(:graphql)

    forward("/graphql", Absinthe.Plug, schema: TuberoseWeb.Schema)
    forward("/graphiql", Absinthe.Plug.GraphiQL, schema: TuberoseWeb.Schema)
  end

  scope "/dev" do
    pipe_through([:browser, :admins_only])

    live_dashboard("/dashboard", metrics: TuberoseWeb.Telemetry)
    forward("/mailbox", Plug.Swoosh.MailboxPreview)
  end

  defp admin_basic_auth(conn, _opts) do
    user = Application.get_env(:tuberose, TuberoseWeb.BasicAuthUser)
    Plug.BasicAuth.basic_auth(conn, username: user[:name], password: user[:password])
  end
end
