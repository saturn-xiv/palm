defmodule Mix.Tasks.Rhododendron.User.AddRole do
  @moduledoc "Usage: `mix rhododendron.user.add_role -u USER_UID -r ROLE_CODE`"
  @shortdoc "Add role to user"

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

      unless Rhododendron.Dao.Role.exists?(role) do
        Rhododendron.Dao.Role.create!(role)
      end

      role = Rhododendron.Repo.get_by!(Rhododendron.Role, code: role)
      Rhododendron.Dao.Role.associate!(user, role)

      Rhododendron.Dao.Log.info!(
        user.id,
        :auth,
        ip,
        %{},
        "Add role #{role.code} by administrator."
      )

      {:ok, true}
    end)

    Logger.info("Done.")
  end
end
