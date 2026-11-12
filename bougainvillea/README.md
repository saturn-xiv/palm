# BOUGAINVILLEA - An online education solution

```bash
$ mix deps.get
# Generate a secret key
$ mix phx.gen.secret

$ mix phx.server

$ iex -S mix phx.server
>
```

## Deployment

```bash
cp etc/staging.env prod.env
source prod.env

bin/bougainvillea eval "Bougainvillea.Release.migrate"
bin/bougainvillea start
```

## Documents

- [gRPC: Supported formats](https://github.com/elixir-grpc/grpc#supported-formats)
