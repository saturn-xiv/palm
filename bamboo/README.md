# BAMBOO - A smart router

## Development

```bash
# install and setup dependencies
$ mix setup
# start Phoenix endpoint: http://localhost:4000
$ mix phx.server
```

- Forwarding remote port to local `ssh -L 14000:localhost:4000 192.168.10.12`

## Deployment

```bash
$ mix phx.gen.secret
$ export SECRET_KEY_BASE="change-me"
$ export DATABASE_URL="postgresql://USER:PASS@HOST:PORT/DATABASE"

$ MIX_ENV=prod mix ecto.migrate
# starting your server in production
$ PORT=4001 MIX_ENV=prod elixir --erl "-detached" -S mix phx.server

# Release mode
$ ./bin/bamboo eval "Bamboo.Release.migrate"
$ PORT=4001  ./bin/bamboo start
```
