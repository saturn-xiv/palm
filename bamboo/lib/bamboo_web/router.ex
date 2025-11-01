defmodule BambooWeb.Router do
  use BambooWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {BambooWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", BambooWeb do
    pipe_through :browser

    scope "/hosts" do
      get "/", HostsController, :index
      get "/:id/edit", HostsController, :edit
      put "/:id", HostsController, :update
    end
    scope "/members" do
      get "/", MembersController, :index
      get "/:id/edit", MembersController, :edit
      put "/:id", MembersController, :update
    end
    scope "/rules" do
      get "/", RulesController, :index
      get "/new", RulesController, :new
      post "/", RulesController, :create
      get "/:id/edit", RulesController, :edit
      put "/:id", RulesController, :update
      delete "/:id", RulesController, :destroy
    end
    scope "/users" do
      get "/sign-in", UsersController, :sign_in
      post "/sign-in", UsersController, :sign_in
      get "/sign-out", UsersController, :sign_out
      get "/profile", UsersController, :profile
      post "/profile", UsersController, :profile
      get "/change-password", UsersController, :change_password
      post "/change-password", UsersController, :change_password
      get "/logs", UsersController, :logs
    end

    get "/nginx.conf", EtcController, :nginx_conf
    get "/service.conf", EtcController, :service_conf

    get "/", PageController, :home
  end

  # Other scopes may use custom stacks.
  # scope "/api", BambooWeb do
  #   pipe_through :api
  # end

  # Enable LiveDashboard and Swoosh mailbox preview in development
  if Application.compile_env(:bamboo, :dev_routes) do
    # If you want to use the LiveDashboard in production, you should put
    # it behind authentication and allow only admins to access it.
    # If your application does not have an admins-only section yet,
    # you can use Plug.BasicAuth to set up some basic authentication
    # as long as you are also using SSL (which you should anyway).
    import Phoenix.LiveDashboard.Router

    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard", metrics: BambooWeb.Telemetry
      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end
