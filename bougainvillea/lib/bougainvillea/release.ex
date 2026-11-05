defmodule Bougainvillea.Release do
  require Logger

  # https://ecto-sql.hexdocs.pm/Ecto.Migrator.html#module-example-running-migrations-in-a-release
  @app :bougainvillea

  def create_user_by_email(name, email, password) do
    Application.ensure_all_started(@app)
    Bougainvillea.Task.User.create_user_by_email(name, email, password)
  end

  def db_seed() do
    Application.ensure_all_started(@app)
    Bougainvillea.Task.DbSeed.run()
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

  def help() do
    priv_dir = :code.priv_dir(:bougainvillea)

    EEx.eval_file(Path.join(priv_dir, "usage.txt.eex"),
      version: Application.spec(:bougainvillea, :vsn)
    )
    |> IO.puts()
  end

  def version do
    IO.puts("Bougainvillea v#{Application.spec(:bougainvillea, :vsn)}")
  end
end
