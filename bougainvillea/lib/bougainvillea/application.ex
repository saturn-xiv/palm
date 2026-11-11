defmodule Bougainvillea.Application do
  # See https://elixir.hexdocs.pm/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      BougainvilleaWeb.Telemetry,
      Bougainvillea.Repo,
      {DNSCluster, query: Application.get_env(:bougainvillea, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Bougainvillea.PubSub},
      # Start a worker by calling: Bougainvillea.Worker.start_link(arg)
      # {Bougainvillea.Worker, arg},
      # Start to serve requests, typically the last entry
      BougainvilleaWeb.Endpoint
    ]

    # See https://elixir.hexdocs.pm/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Bougainvillea.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    BougainvilleaWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
