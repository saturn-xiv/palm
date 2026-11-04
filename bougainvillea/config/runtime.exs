import Config

if config_env() == :prod do
  config_file = "#{System.get_env("BOUGAINVILLEA_ENV", "config")}.toml"

  {:ok, buf} = File.read(config_file)
  {:ok, doc} = TomlElixir.decode(buf)

  config :bougainvillea, Bougainvillea.Repo,
    hostname: doc["postgresql"]["host"],
    port: doc["postgresql"]["port"],
    database: doc["postgresql"]["db-name"],
    username: doc["postgresql"]["user"],
    password: doc["postgresql"]["password"],
    pool_size: doc["postgresql"]["pool-size"]

  config :bougainvillea, :redis,
    host: doc["redis"]["host"],
    port: doc["redis"]["port"],
    db: doc["redis"]["db"],
    pool_size: doc["redis"]["pool-size"]

  config :bougainvillea, :rabbitmq,
    host: doc["rabbitmq"]["host"],
    port: doc["rabbitmq"]["port"],
    user: doc["rabbitmq"]["user"],
    password: doc["rabbitmq"]["password"],
    virtual_host: doc["rabbitmq"]["virtual-host"]

  config :bougainvillea, :opensearch,
    host: doc["opensearch"]["host"],
    namespace: doc["opensearch"]["namespace"]

  config :bougainvillea, :loquat, host: doc["loquat"]["host"], port: doc["loquat"]["port"]
  config :bougainvillea, :dahlia, host: doc["dahlia"]["host"], port: doc["dahlia"]["port"]
  config :bougainvillea, :marigold, host: doc["marigold"]["host"], port: doc["marigold"]["port"]
end
