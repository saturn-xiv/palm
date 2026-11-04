import Config

config :bougainvillea, Bougainvillea.Repo,
  hostname: "127.0.0.1",
  port: 5432,
  database: "bougainvillea_test",
  username: "www",
  password: "change-me",
  pool_size: 8

config :logger, level: :info
