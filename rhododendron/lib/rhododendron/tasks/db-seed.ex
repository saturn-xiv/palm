defmodule Mix.Tasks.Rhododendron.Db.Seed do
  @moduledoc "Usage: `mix rhododendron.db.seed`"
  @shortdoc "Loads data from filesystem without deleting existing records"

  use Mix.Task
  require Logger
  import Ecto.Query

  @impl Mix.Task
  @requirements ["app.start"]
  def run(_args) do
    if Rhododendron.Dao.Currency.count() == 0 do
      {:ok, %{total: total, inserted: inserted}} =
        Rhododendron.Repo.transact(fn ->
          Rhododendron.Dao.Currency.load_from_one_xml(
            Application.app_dir(
              Application.get_application(__MODULE__),
              Path.join(["priv", "iso4217", "list-one.xml"])
            )
          )
        end)

      Logger.info("#{total} total found, #{inserted} inserted.")
    end

    {:ok, %{total: total, inserted: inserted}} =
      Rhododendron.Repo.transact(fn ->
        Rhododendron.Dao.Locale.load_from_yml(
          Application.app_dir(
            Application.get_application(__MODULE__),
            Path.join(["priv", "locales"])
          )
        )
      end)

    Logger.info("#{total} total found, #{inserted} inserted.")

    if Rhododendron.Repo.one!(from t in Rhododendron.Role, select: count()) == 0 do
      Rhododendron.Repo.transact(fn ->
        %Rhododendron.Role{
          code: Rhododendron.Dao.Role.topmost(),
          left: 1,
          right: 2
        }
        |> Rhododendron.Repo.insert!()

        Enum.each(
          [Rhododendron.Dao.Role.root(), Rhododendron.Dao.Role.administrator()],
          fn it ->
            Rhododendron.Dao.Role.create!(it)
          end
        )

        {:ok, true}
      end)
    end

    Logger.info("Done.")
  end
end
