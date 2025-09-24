# Usage

- Start [hyacinth](../hyacinth/)

```bash
# build package
$ mvn clean
$ mvn package

# install your protocol
$ cp target/crocus-2025.8.15.jar ../hyacinth/libs/
```

- Start backends

```bash
./lavender/lavender -d -c lavender.toml rpc -p 10080
./phlox/phlox -d -c phlox.toml rpc -p 18081
```

- Test clients: `./*-test.sh`
