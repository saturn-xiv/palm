defmodule Mix.Tasks.Rhododendron.User.List do
  @moduledoc "Usage: `mix rhododendron.user.user.list`"
  @shortdoc "List all users"

  use Mix.Task
  import Ecto.Query

  @impl Mix.Task
  @requirements ["app.start"]
  def run(_) do
    email_users =
      Rhododendron.Repo.all(
        from(t in Rhododendron.EmailUser, order_by: [asc: t.email], preload: :user)
      )

    IO.puts(
      "#{"UID" |> String.pad_trailing(36, " ")} #{"Name<Email>" |> String.pad_trailing(36, " ")}#{"Current Sign In IP" |> String.pad_trailing(20, " ")}Current Sign In At"
    )

    Enum.each(email_users, fn it ->
      IO.puts(
        "#{it.user.uid} #{"#{it.name}<#{it.email}>" |> String.pad_trailing(36, " ")}#{(it.user.current_sign_in_ip || "n/a") |> String.pad_trailing(20, " ")}\t#{it.user.current_sign_in_at || "n/a"}"
      )
    end)
  end
end
