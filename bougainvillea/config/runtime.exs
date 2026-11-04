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
end
