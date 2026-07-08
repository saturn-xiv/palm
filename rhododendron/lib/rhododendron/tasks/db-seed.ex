defmodule Mix.Tasks.Rhododendron.Db.Seed do
  @moduledoc "Usage: `mix rhododendron.db.seed`"
  @shortdoc "Loads data without deleting existing records"

  require Logger
  use Mix.Task

  @impl Mix.Task
  @requirements ["app.start"]
  def run(_args) do
    if Rhododendron.Dao.Currency.count() == 0 do
      {:ok, %{total: total, inserted: inserted}} =
        Rhododendron.Repo.transact(fn ->
          Rhododendron.Dao.Currency.load_from_one_xml(
            Application.app_dir(
              Application.get_application(__MODULE__),
              "priv/iso4217/list-one.xml"
            )
          )
        end)

      Logger.info("Load currencies, #{total} total found, #{inserted} inserted")
    end

    Logger.info("Load locales")
    Logger.info("Done.")
  end
end
