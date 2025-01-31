# JASMINE - backend services

## Development

```bash
$ sudo pacman -S elixir inotify-tools
$ elixir -v
$ mix archive.install hex phx_new

$ psql -U postgres -h localhost
> CREATE USER www WITH PASSWORD 'change-me';
> CREATE DATABASE jasmine_dev WITH OWNER www;

$ mix ecto.gen.migration create_xxx_yyy
$ mix phx.gen.schema --no-migration Xxx.Yyy xxx_yyy
$ mix ecto.migrate
$ mix ecto.migrations
$ mix ecto.rollback

# populating the database
$ mix run priv/repo/seeds.exs

$ mix setup
# Now you can visit http://localhost:4000 from your browser.
$ mix phx.server
```

## Learn more

- [Phoenix](https://hexdocs.pm/phoenix/installation.html)
- [Ecto](https://hexdocs.pm/ecto/getting-started.html)
- [Tink Cryptographic Library](https://developers.google.com/tink/getting-started)
