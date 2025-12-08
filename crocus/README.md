# Usage

- Start [hyacinth](../hyacinth/)

```bash
mvn clean
mvn package -Dmaven.test.skip=true
cp target/crocus-*.jar ../hyacinth/libs/
```

- Start backends

```bash
daisy -d -c daisy.toml rpc -p 18001
tulip -d -c tulip.toml rpc -p 18002
```

- Test clients: `./*-test.sh`
