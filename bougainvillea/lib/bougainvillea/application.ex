defmodule Bougainvillea.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      Bougainvillea.Repo,
      {Plug.Cowboy,
       scheme: :http,
       plug: Bougainvillea.Router,
       options: [port: Application.fetch_env!(:bougainvillea, :http_port)]}
    ]

    opts = [strategy: :one_for_one, name: Bougainvillea.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
