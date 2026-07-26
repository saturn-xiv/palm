# WISTERIA

- RabbitMq

  ```bash
  sudo rabbitmqctl add_user www "change-me"
  sudo rabbitmqctl add_vhost wisteria.dev
  sudo rabbitmqctl set_permissions -p wisteria.dev www ".*" ".*" ".*"
  ```
