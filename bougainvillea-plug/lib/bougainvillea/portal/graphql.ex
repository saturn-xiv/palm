defmodule Bougainvillea.Portal.Graphql do
  require Logger

  defmodule Session do
    defstruct id: nil, name: nil, lang: nil, user: %{id: nil, code: nil, type: nil}

    def administrator?(%Session{id: id}) do
      {:ok, it} = Dahlia.Rbac.Enforcer.get_implicit_roles_for_user(id)
      it.administrator
    end

    def root?(%Session{id: id}) do
      {:ok, it} = Dahlia.Rbac.Enforcer.get_implicit_roles_for_user(id)
      it.root
    end

    def has?(%Session{id: id}, role) do
      {:ok, it} = Dahlia.Rbac.Enforcer.get_implicit_roles_for_user(id)
      (role |> to_string()) in it.others
    end
  end

  defimpl String.Chars, for: Session do
    def to_string(%Session{id: id, name: name, user: user}) do
      "User(#{name}, #{id})-(#{user.type}, #{user.code})"
    end
  end

  def current_user(token, audience \\ :SIGN_IN) do
    {:ok, subject, payload} =
      Loquat.Jwt.verify(token, Bougainvillea.Portal.token_issuer(), audience |> to_string())

    payload = Protobuf.decode(payload, Palm.Portal.V1.UserToken.Payload)

    case payload.type do
      :EMAIL ->
        case Bougainvillea.Repo.get_by(Bougainvillea.Portal.Models.EmailUser, sn: subject)
             |> Bougainvillea.Repo.preload(:user) do
          nil ->
            {:error, "User #{subject} not found"}

          x when x.deleted_at or x.user.deleted_at ->
            {:error, "User #{x} is deleted"}

          x when x.locked_at or x.user.locked_at ->
            {:error, "User #{x} is locked"}

          x when not x.confirmed_at ->
            {:error, "User #{x} isn't confirmed yet"}

          x ->
            Logger.debug("Current user #{x}")

            {:ok,
             %Session{
               id: x.user.id,
               lang: x.user.lang,
               name: x.name,
               user: %{id: x.id, code: x.email, type: payload.type}
             }}
        end

      v ->
        {:error, "Unimplemented #{v} yet"}
    end
  end

  def succeed() do
    %{created_at: DateTime.utc_now(:microsecond)}
  end
end
