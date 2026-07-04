defmodule Rhododendron.Application do
  # See https://elixir.hexdocs.pm/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      RhododendronWeb.Telemetry,
      Rhododendron.Repo,
      {DNSCluster, query: Application.get_env(:rhododendron, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Rhododendron.PubSub},
      # Start a worker by calling: Rhododendron.Worker.start_link(arg)
      # {Rhododendron.Worker, arg},
      # Start to serve requests, typically the last entry
      RhododendronWeb.Endpoint
    ]

    # See https://elixir.hexdocs.pm/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Rhododendron.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    RhododendronWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
