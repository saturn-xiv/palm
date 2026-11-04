import Config

config :bougainvillea, ecto_repos: [Bougainvillea.Repo]

import_config "#{config_env()}.exs"
