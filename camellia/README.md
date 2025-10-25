# USAGE

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE camellia OWNER www;
```

```bash
mvn clean
mvn package -Dmaven.test.skip=true
# --enable-native-access=ALL-UNNAMED
java -jar target/camellia-2025.10.25.jar --spring.profiles.active=pgsql
```

## Documents

- [The Jinterface Package](https://www.erlang.org/doc/apps/jinterface/jinterface_users_guide.html)
