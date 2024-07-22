# PALM

A total free education &amp; translation solution.

## Usage

```bash
$ git clone https://github.com/saturn-xiv/palm.git ~/workspace/palm
$ cd ~/workspace/palm
$ git submodule update --init --recursive
$ podman run --rm -it --events-backend=file --network host -v $PWD:/workspace:z ubuntu:latest
> cd /workspace
> ./scripts/loquat.sh
```

## Projects

- `loquat` encrypt thrift-rpc service(by Google Tink)
- `petunia` protocol definition files
- `gourd` cpp protocols of thrift
- `lemon` cpp protocols of grpc
- `coconut` backup tools
- `tuberose` cpp application
