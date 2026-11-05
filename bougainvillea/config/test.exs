import Config

config :bougainvillea, Bougainvillea.Repo,
  hostname: "127.0.0.1",
  port: 5432,
  database: "bougainvillea_test",
  username: "www",
  password: "change-me",
  pool_size: 8

config :bougainvillea, :token, issuer: "bougainvillea.test"

config :bougainvillea, :http,
  cookie_key:
    "836LyhhuxhL282SxrDCEELtB/kkj+Mc1Wgo2NJpCXHO+HbXFE2B3U54FHsyuEwjRTLj5zPbmnrB8xy52hU8Y5hCNY04DZNugFa1YNoh/neGXpcnj+iAkhd7r+fkWIixVs8JQ8F2Wt+3YI6xgeyw4T0HaQ9aOLBTQqpxvaZ/guAY="

config :bougainvillea, :redis, host: "127.0.0.1", port: 6379, db: 2, pool_size: 8

config :bougainvillea, :rabbitmq,
  host: "127.0.0.1",
  port: 5672,
  user: "www",
  password: "change-me",
  virtual_host: "bougainvillea.test"

config :bougainvillea, :opensearch,
  endpoint: "http://127.0.0.1:9200",
  namespace: "bougainvillea.dev"

config :bougainvillea, :seaweedfs, endpoint: "http://127.0.0.1:9333"
config :bougainvillea, :loquat, endpoint: "127.0.0.1:11001"
config :bougainvillea, :dahlia, endpoint: "127.0.0.1:11002"
config :bougainvillea, :marigold, endpoint: "127.0.0.1:11004"

config :logger, level: :info
