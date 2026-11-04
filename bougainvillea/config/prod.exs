import Config

config :bougainvillea, Bougainvillea.Repo,
  database: "bougainvillea_prod",
  username: "www",
  password: "change-me",
  hostname: "127.0.0.1",
  port: 5432,
  pool_size: 8

config :logger, level: :info
