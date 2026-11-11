defmodule Mix.Tasks.Bougainvillea.DbSeed do
  use Mix.Task

  @shortdoc "Load the seed data from priv"

  @impl true
  def run(_args) do
    Mix.Task.run("app.start")
    Bougainvillea.Tasks.DbSeed.run()
  end
end
