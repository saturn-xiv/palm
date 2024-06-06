defmodule Mix.Tasks.Tuberose.User.List do
  require Logger

  use Mix.Task

  @shortdoc "List all users"

  @moduledoc """
  Usage:
  mix tuberose.user.list
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(_args) do
    email_users = Tuberose.EmailUser |> Tuberose.Repo.all() |> Tuberose.Repo.preload(:user)
    IO.puts("UID Email Nickname Real name Updated At")

    Enum.map(email_users, fn it ->
      IO.puts("#{it.user.uid} #{it.email} #{it.nickname} #{it.real_name} #{it.updated_at}")
    end)
  end
end
