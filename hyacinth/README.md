# HYACINTH

## Usage

```bash
mvn clean
mvn package -Dmaven.test.skip=true

java --enable-native-access=ALL-UNNAMED -cp 'target/hyacinth-2025.12.8.jar:libs/*' com.github.saturn_xiv.palm.hyacinth.App -c config.toml -p 8180
```

## Testing

```bash
curl -v -X GET "http://127.0.0.1:8180/health-check"
```

## Documents

- [Application properties](https://docs.spring.io/spring-boot/appendix/application-properties/index.html)
- [The Executable Jar Format](https://docs.spring.io/spring-boot/specification/executable-jar/launching.html)
