# WISTERIA - An online education solution

## Setup

- PostgreSql

  ```sql
  CREATE DATABASE wisteria_dev WITH OWNER www;
  ```

  ```bash
  cd wisteria/db/
  dbmate up
  ```

- RabbitMq

  ```bash
  sudo rabbitmqctl add_vhost wisteria.dev
  sudo rabbitmqctl set_permissions -p wisteria.dev www ".*" ".*" ".*"
  ```

## Running

```bash
cargo build

RUST_LOG=debug,h2=info,lapin=info,hyper_util=info,reqwest=info ./target/debug/wisteria -c wisteria/config.toml http -p 4000
```
