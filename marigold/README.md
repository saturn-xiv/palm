# MARIGOLD - WechatPay services

## Setup

- PostgreSql

  ```sql
  CREATE DATABASE marigold_dev WITH OWNER www;
  ```

- RabbitMq

  ```bash
  sudo rabbitmqctl add_vhost marigold.dev
  sudo rabbitmqctl set_permissions -p marigold.dev www ".*" ".*" ".*"
  ```

## Usage

```bash
# clean & build
mvn clean
mvn package -Dmaven.test.skip=true
# running on development mode
java -jar target/marigold-*.jar --spring.config.name=staging
```

## Resources

- [Maven Git Versioning Extension](https://github.com/qoomon/maven-git-versioning-extension)
- [External Application Properties](https://docs.spring.io/spring-boot/reference/features/external-config.html#features.external-config.files)
