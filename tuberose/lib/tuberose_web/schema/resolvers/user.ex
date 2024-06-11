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
        where: p.nickname == ^user or p.email == ^user
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
        %{type: :email, id: it.id},
        it.real_name,
        Tuberose.EmailUser.gravatar(it.email),
        ttl
      )

    {:ok, response}
  end

  defp sign_in(ip, user, provider, name, avatar, ttl) do
    user = Tuberose.Repo.get(Tuberose.User, user)

    if user.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    if user.locked_at do
      raise ArgumentError, message: "User isn't locked"
    end

    Logger.info("user (#{provider.type}, #{provider.id}, #{name}) sign in")
    avatar = if user.avatar, do: user.avatar, else: avatar
    name = if user.name, do: user.name, else: avatar

    rbac = %{
      is_administrator: Tuberose.Atropa.Client.administrator?(user.id),
      is_root: Tuberose.Atropa.Client.root?(user.id),
      roles: Tuberose.Atropa.Client.implicit_roles(user.id),
      permissions: Tuberose.Atropa.Client.implicit_permissions(user.id)
    }

    uid = Ecto.UUID.generate()

    not_before = %Google.Protobuf.Timestamp{
      seconds: (DateTime.utc_now() |> DateTime.to_unix()) - 1
    }

    expires_at = %Google.Protobuf.Timestamp{
      seconds: DateTime.utc_now() |> Timex.shift(seconds: ttl) |> DateTime.to_unix()
    }

    token =
      Tuberose.Atropa.Client.jwt_sign(
        to_string(:tuberose),
        uid,
        [to_string(:sign_in)],
        not_before,
        expires_at
      )

    {:ok, oauth} =
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

        %Tuberose.UserSession{
          user_id: user.id,
          uid: uid,
          provider_type: to_string(provider.type),
          provider_id: provider.id,
          ip: ip,
          expires_at: DateTime.utc_now() |> Timex.shift(seconds: ttl)
        }
        |> Tuberose.Repo.insert()

        %Tuberose.Log{
          user_id: user.id,
          plugin: "core",
          ip: ip,
          level: "info",
          resource_type: "#{provider.type}_user",
          message: "signed in by #{provider.type}"
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
      token: token,
      user:
        %{
          name: name,
          avatar: avatar,
          provider_type: to_string(provider.type),
          lang: user.lang,
          timezone: user.timezone
        }
        |> Map.merge(oauth)
        |> Map.merge(rbac)
    }
  end
end
