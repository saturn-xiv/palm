defmodule Mix.Tasks.Bougainvillea.ListUser do
  use Mix.Task

  @shortdoc "List all user"

  @impl true
  def run(_args) do
    Mix.Task.run("app.start")
    Bougainvillea.Tasks.User.list()
  end
end
