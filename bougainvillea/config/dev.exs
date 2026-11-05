import Config

config :bougainvillea, Bougainvillea.Repo,
  hostname: "127.0.0.1",
  port: 5432,
  database: "bougainvillea_dev",
  username: "www",
  password: "change-me",
  pool_size: 8

config :bougainvillea,
  http_cookie_key:
    "NLOwbwv8W5HHioSG2zuUIzu0DkLn3FxfrJlKXFqOcka222/9DV/uDsnwtvk2JuBIPj+FjDoaG67QyxE2EoG0Xez3Tixg0Rgwr4ToFLKBqlTipQ/q5CpNLW8ujvsl1D5jYdZIH9BtgvkGPxC0NzsIhxbaro5l9sdM4fEYIVWjTfY="

config :bougainvillea, :redis, host: "127.0.0.1", port: 6379, db: 1, pool_size: 8

config :bougainvillea, :rabbitmq,
  host: "127.0.0.1",
  port: 5672,
  user: "www",
  password: "change-me",
  virtual_host: "bougainvillea.dev"

config :bougainvillea, :opensearch,
  endpoint: "http://127.0.0.1:9200",
  namespace: "bougainvillea.dev"

config :bougainvillea, :seaweedfs, endpoint: "http://127.0.0.1:9333"
config :bougainvillea, :loquat, endpoint: "127.0.0.1:11001"
config :bougainvillea, :dahlia, endpoint: "127.0.0.1:11002"
config :bougainvillea, :marigold, endpoint: "127.0.0.1:11004"

config :logger, level: :debug
