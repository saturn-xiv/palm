# Usage

- Start [hyacinth](../hyacinth/)

```bash
mvn clean
mvn package -Dmaven.test.skip=true
cp target/crocus-*.jar ../camellia/libs/
```

- Start backends

```bash
daisy -d -c daisy.toml rpc -p 10080
tulip -d -c tulip.toml rpc -p 18081
```

- Test clients: `./*-test.sh`
