# WISTERIA

- RabbitMq

  ```bash
  sudo rabbitmqctl add_user www "change-me"
  sudo rabbitmqctl add_vhost wisteria.dev
  sudo rabbitmqctl set_permissions -p wisteria.dev www ".*" ".*" ".*"
  ```

## Running

```bash
cargo build
RUST_LOG=debug,h2=info,lapin=info ../target/debug/wisteria -c config.toml http -p 4000
```
