# HIBISCUS - A BEAM node server for bougainvillea

## Setup

- PostgreSql

  ```sql
  CREATE USER www WITH PASSWORD 'change-me';
  CREATE DATABASE hibiscus_dev WITH OWNER www;
  ```

- RabbitMq

  ```bash
  sudo rabbitmqctl add_user www "change-me"
  sudo rabbitmqctl add_vhost hibiscus.dev
  sudo rabbitmqctl set_permissions -p hibiscus.dev www ".*" ".*" ".*"
  ```

## Usage

```bash
cp ~/.asdf/installs/erlang/29.0.3/lib/jinterface-1.16/priv/OtpErlang.jar lib/com/ericsson/jinterface/1.16/jinterface-1.16.jar

mvn clean
mvn package -Dmaven.test.skip=true
java -jar target/hibiscus-*.jar --spring.config.name=staging
```

```elixir
defmodule Hibiscus.Argon2 do
  def encode(password) do
    Node.start(:"elixir-node@127.0.0.1")
    Node.set_cookie(:"secret-cookie")

    target = {:argon2, :"java-node@127.0.0.1"}

    send(target, {self(), :encode, password})

    receive do
      result -> result
    after
      5000 -> {:error, "Timeout waiting for Hibiscus node."}
    end
  end
end
```
