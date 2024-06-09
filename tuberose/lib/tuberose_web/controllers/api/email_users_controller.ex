defmodule TuberoseWeb.Api.EmailUsersController do
  use TuberoseWeb, :controller

  import Ecto.Query
  require Logger

  def sign_in(conn, _params) do
    # TODO
    json(conn, %{id: "in"})
  end

  def sign_up(conn, %{
        "home" => home,
        "locale" => locale,
        "timezone" => timezone,
        "real_name" => real_name,
        "nickname" => nickname,
        "email" => email,
        "password" => password
      }) do
    email = Tuberose.Validation.email!(email)
    locale = Tuberose.Validation.language_code!(locale)
    home = Tuberose.Validation.url!(home)
    timezone = Tuberose.Validation.timezone!(timezone)
    password = Tuberose.Validation.password!(password)
    nickname = Tuberose.Validation.code!(nickname, 3)
    real_name = Tuberose.Validation.label!(real_name, 2)

    avatar = Tuberose.EmailUser.gravatar(email)
    {password, salt} = Tuberose.Atropa.Client.hmac_sign(password, 16)

    Logger.info("create user #{real_name}<#{email}>")

    Tuberose.Repo.transaction(fn ->
      if Tuberose.Repo.get_by(Tuberose.EmailUser, email: email) do
        raise ArgumentError, message: gettext("Email already exists")
      end

      if Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: nickname) do
        raise ArgumentError, message: gettext("Nickname already exists")
      end

      uid = Ecto.UUID.generate()
      %Tuberose.User{uid: uid, lang: locale, timezone: timezone} |> Tuberose.Repo.insert()
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
        ip: conn.assigns.client_ip,
        level: "info",
        resource_type: "email_user",
        message: gettext("signed up by email")
      }
      |> Tuberose.Repo.insert()
    end)

    send_email(email, home, :confirm)
    json(conn, %{})
  end

  def confirm_by_email(conn, %{
        "home" => home,
        "user" => user
      }) do
    user = String.trim(user) |> String.downcase()
    home = Tuberose.Validation.url!(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, gettext("user isn't exists")
    end

    if it.deleted_at do
      raise ArgumentError, gettext("user is disabled")
    end

    if it.confirmed_at do
      raise ArgumentError, gettext("user is active")
    end

    send_email(it.email, home, :confirm)
    json(conn, %{})
  end

  def confirm_by_token(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def unlock_by_email(conn, %{
        "home" => home,
        "user" => user
      }) do
    user = String.trim(user) |> String.downcase()
    home = Tuberose.Validation.url!(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:user_id, :email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, gettext("user isn't exists")
    end

    if it.deleted_at do
      raise ArgumentError, gettext("user is disabled")
    end

    unless it.confirmed_at do
      raise ArgumentError, gettext("user is inactive")
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, gettext("user is disabled")
    end

    unless ur.locked_at do
      raise ArgumentError, gettext("user isn't locked")
    end

    send_email(it.email, home, :unlock)

    json(conn, %{})
  end

  def unlock_by_token(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def forgot_password(conn, %{
        "home" => home,
        "user" => user
      }) do
    user = String.trim(user) |> String.downcase()
    home = Tuberose.Validation.url!(home)

    it =
      from(p in Tuberose.EmailUser,
        where: p.nickname == ^user or p.email == ^user,
        select: map(p, [:user_id, :email, :real_name, :deleted_at, :confirmed_at])
      )
      |> first
      |> Tuberose.Repo.one()

    unless it do
      raise ArgumentError, gettext("user isn't exists")
    end

    if it.deleted_at do
      raise ArgumentError, gettext("user is disabled")
    end

    unless it.confirmed_at do
      raise ArgumentError, gettext("user is inactive")
    end

    ur = Tuberose.Repo.get(Tuberose.User, it.user_id)

    if ur.deleted_at do
      raise ArgumentError, gettext("user is disabled")
    end

    if ur.locked_at do
      raise ArgumentError, gettext("user is locked")
    end

    send_email(it.email, home, :reset_password)
    json(conn, %{})
  end

  def reset_password(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def get_profile(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def set_profile(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def change_password(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  defp send_email(email, _home, action) do
    email_user = Tuberose.Repo.get_by(Tuberose.EmailUser, email: email)
    Logger.info("send a #{action} email to #{email_user.real_name}<#{email_user.email}>")
    # TODO
  end
end
