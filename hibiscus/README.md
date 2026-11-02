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
