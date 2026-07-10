defmodule Mix.Tasks.Rhododendron.User.CreateByEmail do
  @moduledoc "Usage: `mix rhododendron.user.create_by_email -n NAME -e EMAIL -p PASSWORD`"
  @shortdoc "Create an email-user"

  use Mix.Task
  require Logger

  @impl Mix.Task
  @requirements ["app.start"]
  def run(args) do
    switches = [name: :string, email: :string, password: :string]
    aliases = [n: :name, e: :email, p: :password]

    # {parsed, remaining_args, []} =
    #   OptionParser.parse(args, strict: switches, aliases: aliases)
    # IO.puts("Parsed: #{inspect(parsed)} Remaining Args: #{inspect(remaining_args)}")

    {parsed, [], []} = OptionParser.parse(args, strict: switches, aliases: aliases)

    {:ok, name} = Rhododendron.Dao.EmailUser.Validators.name(parsed[:name])
    {:ok, email} = Rhododendron.Dao.EmailUser.Validators.email(parsed[:email])
    {:ok, password} = Rhododendron.Dao.EmailUser.Validators.password(parsed[:password])

    if Rhododendron.Dao.EmailUser.exists?(email) do
      Logger.error("User #{email} already exists")
    else
      ip = Rhododendron.Session.ipv4()

      Rhododendron.Repo.transact(fn ->
        Rhododendron.Dao.EmailUser.create!(name, email, password)

        it = Rhododendron.Repo.get_by!(Rhododendron.EmailUser, email: email)
        Rhododendron.Dao.EmailUser.confirm!(it)
        Rhododendron.Dao.Log.info!(it.user_id, :auth, ip, %{}, "Created by administrator.")
        {:ok, true}
      end)
    end

    Logger.info("Done.")
  end
end
