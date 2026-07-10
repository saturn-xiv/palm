defmodule Mix.Tasks.Rhododendron.User.SetPasswordByEmail do
  @moduledoc "Usage: `mix rhododendron.user.set_password_by_email -e EMAIL -p PASSWORD`"
  @shortdoc "Set email user's password"

  use Mix.Task
  require Logger

  @impl Mix.Task
  @requirements ["app.start"]
  def run(args) do
    switches = [email: :string, password: :string]
    aliases = [e: :email, p: :password]

    {parsed, [], []} = OptionParser.parse(args, strict: switches, aliases: aliases)
    {:ok, email} = Rhododendron.Dao.EmailUser.Validators.email(parsed[:email])
    {:ok, password} = Rhododendron.Dao.EmailUser.Validators.password(parsed[:password])
    ip = Rhododendron.Session.ipv4()

    Rhododendron.Repo.transact(fn ->
      it = Rhododendron.Repo.get_by!(Rhododendron.EmailUser, email: email)
      Rhododendron.Dao.EmailUser.set_password!(it, password)
      Rhododendron.Dao.Log.info!(it.user_id, :auth, ip, %{}, "Reset password by administrator.")
      {:ok, true}
    end)

    Logger.info("Done.")
  end
end
