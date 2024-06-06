defmodule Mix.Tasks.Tuberose.Role.Exempt do
  require Logger

  use Mix.Task

  @shortdoc "Exempt role from role"

  @moduledoc """
  Usage:
  mix tuberose.role.exempt --user UID --role ROLE
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    {parsed, _} = OptionParser.parse!(args, strict: [user: :string, role: :string])

    unless parsed[:user] do
      raise ArgumentError, message: "empty user"
    end

    unless parsed[:role] do
      raise ArgumentError, message: "empty role"
    end

    user = Tuberose.Repo.get_by(Tuberose.User, uid: parsed[:user])

    user_it = %Palm.Atropa.V1.PolicyUsersResponse.Item{by: {:i, user.id}}
    role_it = %Palm.Atropa.V1.PolicyRolesResponse.Item{by: {:code, parsed[:role]}}

    role_it =
      if parsed[:role] == "administrator",
        do: %Palm.Atropa.V1.PolicyRolesResponse.Item{
          by: {:administrator, %Palm.Atropa.V1.PolicyRolesResponse.Item.Administrator{}}
        },
        else: role_it

    role_it =
      if parsed[:role] == "root",
        do: %Palm.Atropa.V1.PolicyRolesResponse.Item{
          by: {:root, %Palm.Atropa.V1.PolicyRolesResponse.Item.Root{}}
        },
        else: role_it

    Tuberose.Atropa.Client.delete_roles_for_user(user_it, [role_it])

    Tuberose.Repo.transaction(fn ->
      %Tuberose.Log{
        user_id: user.id,
        plugin: "core",
        ip: "localhost",
        level: "warning",
        resource_type: "user",
        message: "exempt from role(#{parsed[:role]})"
      }
      |> Tuberose.Repo.insert()
    end)
  end
end
