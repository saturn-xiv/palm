# HIBISCUS - A BEAM node server for bougainvillea

## Usage

```bash
cp ~/.asdf/installs/erlang/29.0.3/lib/jinterface-1.16/priv/OtpErlang.jar lib/com/ericsson/jinterface/1.16/jinterface-1.16.jar

mvn clean
mvn package
java -jar target/hibiscus-v0.1.0-126-d202d7c-DIRTY.jar otp-node -n java_node@127.0.0.1 -m hibiscus_mailbox -c secret_cookie config.yml
```

## How to start the Hibiscus application

1. Run `mvn clean install` to build your application
1. Start application with `java -jar target/hibiscus-1.0-SNAPSHOT.jar server config.yml`
1. To check that your application is running enter url `http://localhost:8080`

## Health Check

To see your applications health enter url `http://localhost:8081/healthcheck`
