defmodule Mix.Tasks.Rhododendron.User.RemoveRole do
  @moduledoc "Usage: `mix rhododendron.user.remove_role -u USER_UID -r ROLE_CODE`"
  @shortdoc "Remove role from user"

  use Mix.Task
  require Logger

  @impl Mix.Task
  @requirements ["app.start"]
  def run(args) do
    switches = [user: :string, role: :string]
    aliases = [u: :user, r: :role]

    {parsed, [], []} = OptionParser.parse(args, strict: switches, aliases: aliases)

    {:ok, role} = Rhododendron.Dao.Role.Validators.code(parsed[:role])
    ip = Rhododendron.Session.ipv4()

    Rhododendron.Repo.transact(fn ->
      user = Rhododendron.Repo.get_by!(Rhododendron.User, uid: parsed[:user])

      role = Rhododendron.Repo.get_by!(Rhododendron.Role, code: role)
      Rhododendron.Dao.Role.disassociate!(role, user)

      Rhododendron.Dao.Log.info!(
        user.id,
        :auth,
        ip,
        %{},
        "Remove role #{role.code} by administrator."
      )

      {:ok, true}
    end)

    Logger.info("Done.")
  end
end
