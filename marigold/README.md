# MARIGOLD - WechatPay services

## setup

- PostgreSql

  ```sql
  CREATE USER www WITH PASSWORD 'change-me';
  CREATE DATABASE marigold_dev WITH OWNER www;
  ```

- RabbitMq

  ```bash
  sudo rabbitmqctl add_user www "change-me"
  sudo rabbitmqctl add_vhost marigold.dev
  sudo rabbitmqctl set_permissions -p marigold.dev www ".*" ".*" ".*"
  ```

## Usage

```bash
# clean & build
mvn clean
mvn package -Dmaven.test.skip=true
# running on develpoment mode
java -jar target/marigold-*.jar
# running on staging mode
java -jar target/marigold-$(git describe --tags --abbrev=7 --always | sed 's/-g/-/')*.jar --spring.config.name=staging
```

## Resources

- [Maven Git Versioning Extension](https://github.com/qoomon/maven-git-versioning-extension)
- [External Application Properties](https://docs.spring.io/spring-boot/reference/features/external-config.html#features.external-config.files)
