defmodule Rhododendron.Dao.Role do
  import Ecto.Query
  import Ecto.Changeset

  @topmost "null"
  @administrator "administrator"
  @root "root"

  def associate!(role, user) do
    role = role |> Rhododendron.Repo.preload(:users)
    users = role.users ++ [user]

    role
    |> change()
    |> put_assoc(:users, users)
    |> Rhododendron.Repo.insert_or_update()
  end

  def disassociate!(role, user) do
    role = role |> Rhododendron.Repo.preload(:users)
    users = Enum.reject(role.users, &(&1.id == user.id))

    role
    |> change()
    |> put_assoc(:users, users)
    |> Rhododendron.Repo.update()
  end

  def exists?(code) do
    Rhododendron.Repo.one!(
      from t in Rhododendron.Role,
        select: count(),
        where: t.code == ^code
    ) > 0
  end

  def create!(code, parent \\ @topmost) do
    parent = Rhododendron.Repo.get_by!(Rhododendron.Role, code: parent)
    kids = parent |> children
    now = DateTime.utc_now(:microsecond)

    if Enum.empty?(kids) do
      Rhododendron.Repo.update_all(
        from(t in Rhododendron.Role,
          where: t.right > ^parent.left,
          update: [
            inc: [right: 2],
            set: [updated_at: ^now]
          ]
        ),
        []
      )

      Rhododendron.Repo.update_all(
        from(t in Rhododendron.Role,
          where: t.left > ^parent.left,
          update: [
            inc: [left: 2],
            set: [updated_at: ^now]
          ]
        ),
        []
      )

      %Rhododendron.Role{
        code: code,
        left: parent.left + 1,
        right: parent.left + 2
      }
      |> Rhododendron.Repo.insert!()
    else
      brother = List.last(kids)

      Rhododendron.Repo.update_all(
        from(t in Rhododendron.Role,
          where: t.right > ^brother.right,
          update: [
            inc: [right: 2],
            set: [updated_at: ^now]
          ]
        ),
        []
      )

      Rhododendron.Repo.update_all(
        from(t in Rhododendron.Role,
          where: t.left > ^brother.right,
          update: [
            inc: [left: 2],
            set: [updated_at: ^now]
          ]
        ),
        []
      )

      %Rhododendron.Role{
        code: code,
        left: brother.right + 1,
        right: brother.right + 2
      }
      |> Rhododendron.Repo.insert!()
    end
  end

  def tree(parent) do
    Rhododendron.Repo.all(
      from t in Rhododendron.Role, where: t.left >= ^parent.left and t.right <= ^parent.right
    )
  end

  def children(parent) do
    Rhododendron.Repo.all(
      from t in Rhododendron.Role, where: t.left > ^parent.left and t.right < ^parent.right
    )
  end

  def topmost() do
    @topmost
  end

  def root() do
    @root
  end

  def administrator() do
    @administrator
  end

  defmodule Validators do
    @code_regex ~r/^[a-z0-9._-]{2,63}$/

    def code(s) do
      s = s |> String.trim() |> String.downcase()

      if s =~ @code_regex do
        {:ok, s}
      else
        {:error, "#{s} isn't a valid role code"}
      end
    end
  end
end
