# USAGE

- Setup database

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE camellia OWNER www;
```

- Start application

```bash
$ mvn clean
$ mvn package -Dmaven.test.skip=true
# --enable-native-access=ALL-UNNAMED
$ cp ~/.asdf/installs/erlang/28.1.1/lib/jinterface-1.15/priv/OtpErlang.jar lib/
$ java -cp 'target/camellia-2025.10.25.jar:lib/*' org.springframework.boot.loader.launch.JarLauncher --spring.profiles.active=pgsql
```

- Testing

```erlang
> code:root_dir().
node().
rpc:call(servernode@byrned.thoughtworks.com, mathserver, add, [1,2]).
```

## Documents

- [The Jinterface Package](https://www.erlang.org/doc/apps/jinterface/jinterface_users_guide.html)
