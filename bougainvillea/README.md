# BOUGAINVILLEA - An online education solution

## Setup

- PostgreSql

  ```sql
  CREATE DATABASE bougainvillea_dev WITH OWNER www;
  ```

- RabbitMq

  ```bash
  sudo rabbitmqctl add_vhost bougainvillea.dev
  sudo rabbitmqctl set_permissions -p bougainvillea.dev www ".*" ".*" ".*"
  ```

## Run on development mode

```bash
mix deps.get
mix compile
iex -S mix

mix ecto.gen.migration create_xxx
mix ecto.migrate
mix ecto.rollback

mix run --no-halt

MIX_ENV=prod mix release --overwrite
_build/prod/rel/bougainvillea/bin/bougainvillea eval 'IO.puts("Hello, Bougainvillea!")'
```

## Run on production mode

```bash
export BOUGAINVILLEA_ENV=staging
export BOUGAINVILLEA_HTTP_PORT=3000
./bin/bougainvillea eval 'Bougainvillea.Release.db_migrate'
./bin/bougainvillea eval 'Bougainvillea.Release.db_status'
./bin/bougainvillea eval 'Bougainvillea.Release.db_rollback(20261102053706)'
./bin/bougainvillea eval 'Bougainvillea.Release.db_seed'

./bin/bougainvillea start
```

## Documents

- [gRPC: Supported formats](https://github.com/elixir-grpc/grpc#supported-formats)
