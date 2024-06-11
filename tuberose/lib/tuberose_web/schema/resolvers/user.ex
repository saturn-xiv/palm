defmodule TuberoseWeb.Resolvers.User do
  require Logger
  import Ecto.Query
  import Ecto.Changeset

  def sign_in_by_email(
        _parent,
        %{
          user: user,
          password: password,
          ttl: ttl
        },
        %{context: context}
      ) do
    user = String.trim(user) |> String.downcase()

    unless ttl >= 60 * 10 and ttl <= 60 * 60 * 24 * 7 do
      raise ArgumentError, message: "bad ttl"
    end

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, message: "User is not exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless it.confirmed_at do
      raise ArgumentError, message: "User is inactive"
    end

    Tuberose.Atropa.Client.hmac_verify(it.password, password, it.salt)

    response =
      sign_in(
        context.client_ip,
        it.user_id,
        :email,
        it.real_name,
        Tuberose.EmailUser.gravatar(it.email)
      )

    {:ok, response}
  end

  defp sign_in(ip, user, provider_type, name, avatar) do
    user = Tuberose.Repo.get(Tuberose.User, user)

    if user.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless user.locked_at do
      raise ArgumentError, message: "User isn't locked"
    end

    Logger.info("user (#{user.id}, #{provider_type}, #{name}) sign in")
    avatar = if user.avatar, do: user.avatar, else: avatar
    name = if user.name, do: user.name, else: avatar

    rbac = %{
      is_administrator: Tuberose.Atropa.Client.administrator?(user.id),
      is_root: Tuberose.Atropa.Client.root?(user.id),
      roles: Tuberose.Atropa.Client.implicit_roles(user.id),
      permissions: Tuberose.Atropa.Client.implicit_permissions(user.id)
    }

    oauth =
      Tuberose.Repo.transaction(fn ->
        Tuberose.Repo.update(
          change(user, %{
            sign_in_count: user.sign_in_count + 1,
            last_sign_in_at: user.current_sign_in_at,
            last_sign_in_ip: user.current_sign_in_ip,
            current_sign_in_at: DateTime.utc_now(),
            current_sign_in_ip: ip
          })
        )

        %Tuberose.Log{
          user_id: user.id,
          plugin: "core",
          ip: ip,
          level: "info",
          resource_type: "email_user",
          message: "signed in by email"
        }
        |> Tuberose.Repo.insert()

        # Tuberose.Repo.one(
        #       from(p in Tuberose.GoogleUser,
        #         select: count()
        #       )
        #     ) > 0
        %{
          has_wechat_mini_program: false,
          has_wechat_oauth2: false,
          has_google: false
        }
      end)

    %{
      name: name,
      avatar: avatar,
      provider_type: provider_type,
      lang: user.lang,
      timezone: user.timezone
    }
    |> Map.merge(oauth)
    |> Map.merge(rbac)
  end
end
