import Config

config :bougainvillea, Bougainvillea.Repo,
  database: "bougainvillea_dev",
  username: "www",
  password: "change-me",
  hostname: "127.0.0.1",
  port: 5432

config :bougainvillea, ecto_repos: [Bougainvillea.Repo]
