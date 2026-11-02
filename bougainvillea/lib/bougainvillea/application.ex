defmodule Bougainvillea.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {Plug.Cowboy, scheme: :http, plug: Bougainvillea.Router, options: [port: 8080]}
      # Starts a worker by calling: Bougainvillea.Worker.start_link(arg)
      # {Bougainvillea.Worker, arg}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Bougainvillea.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
