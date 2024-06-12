# ALOE - A smart router inspired by OpenWrt

## USAGE

- Development

```bash
cd aloe
mix deps.get
mix ecto.create
mix phx.server
```

- Build for production

```bash
$ podman run --rm -it --events-backend=file --network host -v $PWD:/workspace:z ubuntu:latest
> cd /workspace
> ./scripts/phoenix.sh aloe
```

- Deployment

```bash
export SECRET_KEY_BASE="really long secret string" # mix phx.gen.secret
export DATABASE_URL="/var/lib/aloe/db"

./bin/migrate
./bin/server
```
