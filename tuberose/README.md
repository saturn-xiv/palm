# Tuberose

## Development

### Usage

- PostgreSql

  ```sql
  > CREATE USER www WITH PASSWORD 'change-me';
  > CREATE DATABASE palm_dev WITH ENCODING = 'UTF8' OWNER www;
  ```

- For ArchLinux

  ```bash
  sudo pacman -S erlang elixir inotify-to
  mix local.hex
  mix archive.install hex phx_new
  mix deps.get
  mix phx.server # http://localhost:4000
  ```

### Upgrade notices

## Deployment

- Build package

  ```bash



  ```

- Starting

  ```bash
  export SECRET_KEY_BASE="really long secret string" # mix phx.gen.secret
  export DATABASE_URL="postgres://www:change-me@127.0.0.1/palm?sslmode=disable"

  ./bin/server
  ```

## Documents

### Background

- [Elixir](https://hexdocs.pm/elixir/introduction.html)
- [Phoenix](https://hexdocs.pm/phoenix/overview.html)
