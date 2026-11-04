defmodule Bougainvillea.Release do
  require Logger

  # https://ecto-sql.hexdocs.pm/Ecto.Migrator.html#module-example-running-migrations-in-a-release
  @app :bougainvillea

  def db_seed() do
    Application.ensure_all_started(@app)
    Bougainvillea.Tasks.DbSeed.run()
    Logger.info("done.")
  end

  def db_migrate() do
    Application.load(@app)

    for repo <- Application.fetch_env!(@app, :ecto_repos) do
      {:ok, _, _} = Ecto.Migrator.with_repo(repo, &Ecto.Migrator.run(&1, :up, all: true))
    end

    Logger.info("done.")
  end

  def db_rollback(version) do
    Application.load(@app)

    {:ok, _, _} =
      Ecto.Migrator.with_repo(Bougainvillea.Repo, &Ecto.Migrator.run(&1, :down, to: version))
  end

  def db_status() do
    Application.load(@app)

    for repo <- Application.fetch_env!(@app, :ecto_repos) do
      IO.puts("=== #{inspect(repo)} ===")

      Ecto.Migrator.with_repo(repo, fn repo ->
        for {status, version, name} <- Ecto.Migrator.migrations(repo) do
          IO.puts("#{version} #{name |> String.pad_trailing(32)} #{status}")
        end
      end)
    end
  end
end
