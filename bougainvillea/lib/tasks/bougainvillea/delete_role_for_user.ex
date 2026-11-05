defmodule Mix.Tasks.Bougainvillea.DeleteRoleForUser do
  use Mix.Task

  @shortdoc "Removes a role from a user(-u/--user, -r/--role)"

  @impl true
  def run(args) do
    Mix.Task.run("app.start")

    {parsed_options, _remaining_args, invalid_options} =
      OptionParser.parse(args,
        strict: [user: :string, role: :string],
        aliases: [u: :user, r: :role]
      )

    if invalid_options != [] do
      Mix.shell().error("Invalid options provided: #{inspect(invalid_options)}")
    end

    user = Keyword.get(parsed_options, :user)
    role = Keyword.get(parsed_options, :role)
    Bougainvillea.Tasks.User.delete_role_for_user(user, role)
  end
end
