defmodule Dahlia.Rbac.Enforcer do
  @administrator Dahlia.Rbac.Roles.administrator()
  @root Dahlia.Rbac.Roles.root()

  def get_implicit_roles_for_user(user) do
    {:ok, ch} = Dahlia.open()

    {:ok, res} =
      Palm.Rbac.V1.EnforcerService.Stub.get_implicit_roles_for_user(ch, user_by_id(user))

    {:ok,
     %Dahlia.Rbac.Roles{
       administrator:
         Enum.any?(res.items, fn x ->
           case x.by do
             {:administrator, _} -> true
             _ -> false
           end
         end),
       root:
         Enum.any?(res.items, fn x ->
           case x.by do
             {:root, _} -> true
             _ -> false
           end
         end),
       others:
         Enum.reduce(res.items, [], fn it, acc ->
           case it.by do
             {:code, v} -> acc ++ [v]
             _ -> acc
           end
         end)
     }}
  end

  def add_role_for_user(user, role) do
    {:ok, ch} = Dahlia.open()

    {:ok, _} =
      Palm.Rbac.V1.EnforcerService.Stub.add_role_for_user(ch, user_role_request(user, role))
  end

  def delete_role_for_user(user, role) do
    {:ok, ch} = Dahlia.open()

    {:ok, _} =
      Palm.Rbac.V1.EnforcerService.Stub.delete_role_for_user(ch, user_role_request(user, role))
  end

  defp user_by_id(id) do
    %Palm.Rbac.V1.Subject.User{by: {:id, id}}
  end

  defp user_role_request(user, role) do
    %Palm.Rbac.V1.UserRoleRequest{
      user: user_by_id(user),
      role:
        case role do
          @administrator ->
            %Palm.Rbac.V1.Subject.Role{
              by: {:administrator, %Palm.Rbac.V1.Subject.Role.Administrator{}}
            }

          @root ->
            %Palm.Rbac.V1.Subject.Role{
              by: {:root, %Palm.Rbac.V1.Subject.Role.Root{}}
            }

          v ->
            %Palm.Rbac.V1.Subject.Role{by: {:code, v}}
        end
    }
  end
end
