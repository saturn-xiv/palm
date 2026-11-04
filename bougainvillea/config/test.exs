import Config

config :bougainvillea, Bougainvillea.Repo,
  hostname: "127.0.0.1",
  port: 5432,
  database: "bougainvillea_test",
  username: "www",
  password: "change-me",
  pool_size: 8

config :bougainvillea, :redis, host: "127.0.0.1", port: 6379, db: 2, pool_size: 8

config :bougainvillea, :rabbitmq,
  host: "127.0.0.1",
  port: 5672,
  user: "www",
  password: "change-me",
  virtual_host: "bougainvillea.test"

config :bougainvillea, :opensearch, host: "http://127.0.0.1:9200", namespace: "bougainvillea.dev"

config :bougainvillea, :loquat, host: "127.0.0.1", port: 11001
config :bougainvillea, :dahlia, host: "127.0.0.1", port: 11002
config :bougainvillea, :marigold, host: "127.0.0.1", port: 11004

config :logger, level: :info
