import Config

config :bougainvillea, ecto_repos: [Bougainvillea.Repo]

config :bougainvillea, :http,
  port: System.get_env("BOUGAINVILLEA_HTTP_PORT", "3000") |> String.to_integer()

import_config "#{config_env()}.exs"
