defmodule Mix.Tasks.Rhododendron.User.CreateByEmail do
  @moduledoc "Usage: `mix rhododendron.user.create_by_email -n NAME -e EMAIL -p PASSWORD`"
  @shortdoc "Create an email-user"

  require Logger
  use Mix.Task

  @impl Mix.Task
  @requirements ["app.start"]
  def run(args) do
    switches = [name: :string, email: :string, password: :string]
    aliases = [n: :name, e: :email, p: :password]

    # {parsed, remaining_args, []} =
    #   OptionParser.parse(args, strict: switches, aliases: aliases)
    # IO.puts("Parsed: #{inspect(parsed)} Remaining Args: #{inspect(remaining_args)}")

    {parsed, [], []} = OptionParser.parse(args, strict: switches, aliases: aliases)

    if Rhododendron.Dao.EmailUser.exists?(parsed[:email]) do
      Logger.error("User #{parsed[:name]} already exists")
    else
      ip = Rhododendron.Session.ipv4()

      Rhododendron.Repo.transact(fn ->
        {_, email} =
          Rhododendron.Dao.EmailUser.create!(parsed[:name], parsed[:email], parsed[:password])

        it = Rhododendron.Repo.get_by!(Rhododendron.EmailUser, email: email)
        Rhododendron.Dao.EmailUser.confirm!(it)
        Rhododendron.Dao.Log.info!(it.user_id, :auth, ip, %{}, "Created by administrator.")
        {:ok, true}
      end)
    end

    Logger.info("Done.")
  end
end
