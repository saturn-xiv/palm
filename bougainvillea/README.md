# BOUGAINVILLEA - An online education solution

## Development

```bash
$ mix deps.get
# Generate a secret key
$ mix phx.gen.secret

$ mix compile
# https://phoenix.hexdocs.pm/ecto.html#using-phx-gen-schema
$ mix ecto.migrate
$ mix phx.server
$ iex -S mix phx.server
>
```

### Issues

-- no route found for GET /assets/css/app.css

```bash
mix tailwind.install https://github.com/tailwindlabs/tailwindcss/releases/download/v4.3.3/tailwindcss-linux-x64
```

## Deployment

```bash
cp etc/staging.env prod.env
set -a && source prod.env && set +a

bin/bougainvillea eval "Bougainvillea.Release.migrate"
PHX_SERVER=true bin/bougainvillea start
```

## Documents

- [gRPC: Supported formats](https://github.com/elixir-grpc/grpc#supported-formats)
