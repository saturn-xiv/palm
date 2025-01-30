defmodule Jasmine.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      JasmineWeb.Telemetry,
      Jasmine.Repo,
      {DNSCluster, query: Application.get_env(:jasmine, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Jasmine.PubSub},
      # Start the Finch HTTP client for sending emails
      {Finch, name: Jasmine.Finch},
      # Start a worker by calling: Jasmine.Worker.start_link(arg)
      # {Jasmine.Worker, arg},
      # Start to serve requests, typically the last entry
      JasmineWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Jasmine.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    JasmineWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
