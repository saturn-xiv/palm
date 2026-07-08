# RHODODENDRON

## Usage

```bash
git clone -b phoenix https://github.com/saturn-xiv/palm.git $HOME/workspace/palm
cd $HOME/workspace/palm/
./scripts/build.sh
```

To start your Phoenix server:

- Run `mix setup` to install and setup dependencies
- Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

Ready to run in production? Please [check our deployment guides](https://phoenix.hexdocs.pm/deployment.html).

## Development

### Database prepare

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE rhododendron_dev WITH OWNER www ENCODING='UTF8';
```

```bash
mix ecto.migrate
mix ecto.migrations
mix ecto.rollback
```

## Testing

```bash
mix test test/rhododendron/crypto_test.exs
```

## Learn more

- [Erl_Interface User's Guide](https://www.erlang.org/doc/apps/erl_interface/ei_users_guide.html)
- [ISO 4217 Currency codes](https://www.iso.org/iso-4217-currency-codes.html)
