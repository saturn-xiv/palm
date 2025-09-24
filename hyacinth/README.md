# Hyacinth

## Usage

```bash
# build target package
$ mvn clean
$ mvn package

# start your application
$ java -jar target/hyacinth-2025.9.24.jar server config.yml -p 8080

$ java -Dlogback.configurationFile=logback.xml -cp 'target/hyacinth-2025.8.15-full.jar:libs/*' com.github.saturn_xiv.palm.hyacinth.App -p 8080 -c config.toml
```

1. Run `mvn clean install` to build your application
1. Start application with `java -jar target/hyacinth-2025.9.24.jar server config.yml`
1. To check that your application is running enter url `http://localhost:8080`

## Health Check

To see your applications health enter url `http://localhost:8081/healthcheck`
