defmodule Mix.Tasks.Bougainvillea.DbSeed do
  use Mix.Task
  require Logger

  @shortdoc "Load the seed data from priv"

  @impl true
  def run(_args) do
    Mix.Task.run("app.start")
    Bougainvillea.Tasks.DbSeed.run()
    Logger.info("done.")
  end
end
