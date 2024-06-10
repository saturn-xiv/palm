defmodule TuberoseWeb.Resolvers.EmailUser do
  require Logger
  import Ecto.Query

  def confirm(_parent, %{user: user, home: home}, _resolution) do
    user = String.trim(user) |> String.downcase()
    {:ok, home} = Tuberose.Validation.url(home)

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

    if it.confirmed_at do
      raise ArgumentError, message: "User is active"
    end

    send_email(it.email, home, :confirm)
    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def unlock(_parent, %{user: user, home: home}, _resolution) do
    user = String.trim(user) |> String.downcase()
    {:ok, home} = Tuberose.Validation.url(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:user_id, :email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, message: "User isn't exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless it.confirmed_at do
      raise ArgumentError, message: "User is inactive"
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless ur.locked_at do
      raise ArgumentError, message: "User isn't locked"
    end

    send_email(it.email, home, :unlock)
    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def forgot_password(_parent, %{user: user, home: home}, _resolution) do
    user = String.trim(user) |> String.downcase()
    {:ok, home} = Tuberose.Validation.url(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:user_id, :email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, message: "User isn't exists"
    end

    if it.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    unless it.confirmed_at do
      raise ArgumentError, message: "User is inactive"
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, message: "User is disabled"
    end

    if ur.locked_at do
      raise ArgumentError, message: "User is locked"
    end

    send_email(it.email, home, :reset_password)
    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def sign_up(
        _parent,
        %{
          real_name: real_name,
          nickname: nickname,
          email: email,
          password: password,
          home: home,
          timezone: timezone
        },
        %{context: context}
      ) do
    {:ok, nickname} = Tuberose.Validation.code(nickname, 3)
    {:ok, real_name} = Tuberose.Validation.label(real_name, 2)
    {:ok, email} = Tuberose.Validation.email(email)
    {:ok, password} = Tuberose.Validation.password(password)
    {:ok, timezone} = Tuberose.Validation.timezone(timezone)
    {:ok, home} = Tuberose.Validation.url(home)

    avatar = Tuberose.EmailUser.gravatar(email)
    {password, salt} = Tuberose.Atropa.Client.hmac_sign(password, 16)

    Logger.info("create user #{real_name}<#{email}>")

    Tuberose.Repo.transaction(fn ->
      if Tuberose.Repo.get_by(Tuberose.EmailUser, email: email) do
        raise ArgumentError, message: "Email already exists"
      end

      if Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: nickname) do
        raise ArgumentError, message: "Nickname already exists"
      end

      uid = Ecto.UUID.generate()
      %Tuberose.User{uid: uid, lang: context.locale, timezone: timezone} |> Tuberose.Repo.insert()
      user = Tuberose.Repo.get_by(Tuberose.User, uid: uid)

      %Tuberose.EmailUser{
        user_id: user.id,
        email: email,
        password: password,
        salt: salt,
        nickname: nickname,
        real_name: real_name,
        avatar: avatar
      }
      |> Tuberose.Repo.insert()

      %Tuberose.Log{
        user_id: user.id,
        plugin: "core",
        ip: context.client_ip,
        level: "info",
        resource_type: "email_user",
        message: "signed up by email"
      }
      |> Tuberose.Repo.insert()
    end)

    send_email(email, home, :confirm)
    {:ok, %{:created_at => DateTime.utc_now()}}
  end

  defp send_email(email, _home, action) do
    email_user = Tuberose.Repo.get_by(Tuberose.EmailUser, email: email)
    Logger.info("send a #{action} email to #{email_user.real_name}<#{email_user.email}>")
    # TODO
  end
end
