# USAGE

```bash
mvn clean
mvn package -Dmaven.test.skip=true
java -jar target/camellia-2025.10.25.jar server config.yml
```

- To see your applications health enter url `http://localhost:8081/healthcheck`

## Documents

- [The Jinterface Package](https://www.erlang.org/doc/apps/jinterface/jinterface_users_guide.html)
