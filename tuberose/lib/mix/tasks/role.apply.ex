defmodule Mix.Tasks.Tuberose.Role.Apply do
  use Mix.Task

  @shortdoc "Apply role to user"

  @moduledoc """
  Generate a jwt token for third-party endpoints.
  Usage:
  mix tuberose.role.apply --subject sss --issuer iii --audience a1 --audience a2 --years 20
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    Mix.shell().info("")
  end
end
