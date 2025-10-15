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
$ java --enable-native-access=ALL-UNNAMED -cp 'target/hyacinth-2025.9.25.jar:libs/*' org.springframework.boot.loader.launch.JarLauncher --spring.profiles.active=dev

```

## Testing

```bash
curl -v -X GET "http://127.0.0.1:8180/health-check"
```

## Documents

- [Application properties](https://docs.spring.io/spring-boot/appendix/application-properties/index.html)
- [The Executable Jar Format](https://docs.spring.io/spring-boot/specification/executable-jar/launching.html)
