# Usage

- Start [hyacinth](../hyacinth/)

```bash
mvn clean && mvn package
cp target/crocus-2025.8.15.jar ../hyacinth/libs/
```

- Start backends

```bash
./lavender/lavender -d -c lavender.toml rpc -p 10080
./phlox/phlox -d -c phlox.toml rpc -p 18081
```

- Curl client test: `./*-test.sh`
