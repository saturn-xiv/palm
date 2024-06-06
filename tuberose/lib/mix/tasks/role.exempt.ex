defmodule Mix.Tasks.Tuberose.Role.Exempt do
  use Mix.Task

  @shortdoc "Exempt role from role"

  @moduledoc """
  Generate a jwt token for third-party endpoints.
  Usage:
  mix tuberose.role.exempt --subject sss --issuer iii --audience a1 --audience a2 --years 20
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    Mix.shell().info("")
  end
end
