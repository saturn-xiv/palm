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
$ java -cp 'target/hyacinth-2025.9.25.jar:libs/*' org.springframework.boot.loader.launch.JarLauncher --spring.profiles.active=dev
```

## Documents

- [Application properties](https://docs.spring.io/spring-boot/appendix/application-properties/index.html)
- [The Executable Jar Format](https://docs.spring.io/spring-boot/specification/executable-jar/launching.html)
