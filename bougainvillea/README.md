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

mix ecto.gen.migration create_xxx
mix ecto.migrate
mix ecto.rollback

mix run --no-halt
```


