# Hyacinth

## Usage

### Setup database

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE hyacinth OWNER www;
```

```bash
# build your application
$ mvn clean
$ mvn package -Dmaven.test.skip=true

# start application
$ java -jar target/hyacinth-2025.9.25.jar --spring.profiles.active=dev

$ java -jar target/hyacinth-2025.9.24.jar server config.yml

# setup postgresql
$ java -jar target/hyacinth-2025.9.24.jar db status --migrations $PWD/db/postgresql/migrations.xml config.yml
$ java -jar target/hyacinth-2025.9.24.jar db migrate --migrations $PWD/db/postgresql/migrations.xml config.yml

# FIXME setup derby
$ java -jar target/hyacinth-2025.9.24.jar db status --migrations $PWD/db/derby/migrations.xml config.yml
$ java -jar target/hyacinth-2025.9.24.jar db migrate --migrations $PWD/db/derby/migrations.xml config.yml

$ java -Dlogback.configurationFile=logback.xml -cp 'target/hyacinth-2025.8.15-full.jar:libs/*' com.github.saturn_xiv.palm.hyacinth.App -p 8080 -c config.toml
```

1. To check that your application is running enter url `http://localhost:1080`

## Health Check

To see your applications health enter url `http://localhost:8181/healthcheck`

## Documents

- [Spring boot application properties](https://docs.spring.io/spring-boot/appendix/application-properties/index.html)
