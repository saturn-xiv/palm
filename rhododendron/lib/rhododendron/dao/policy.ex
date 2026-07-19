defmodule Rhododendron.Dao.Policy do
  def get_implicit_permissions_for_user(user) do
    user = user |> Rhododendron.Repo.preload(:policies)

    items =
      Enum.reduce(Rhododendron.Dao.Role.get_implicit_roles_for_user(user), [], fn el, acc ->
        role = el |> Rhododendron.Repo.preload(:policies)
        acc ++ role.policies
      end)

    Enum.uniq_by(items ++ user.policies, fn x -> x.id end)
  end
end
