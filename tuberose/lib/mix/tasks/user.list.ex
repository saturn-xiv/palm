defmodule Mix.Tasks.Tuberose.User.List do
  use Mix.Task

  @shortdoc "List all users"

  @moduledoc """
  Generate a jwt token for third-party endpoints.
  Usage:
  mix tuberose.user.list --subject sss --issuer iii --audience a1 --audience a2 --years 20
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    Mix.shell().info("")
  end
end
