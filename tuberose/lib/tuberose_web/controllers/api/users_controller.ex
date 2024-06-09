defmodule TuberoseWeb.Api.UsersController do
  use TuberoseWeb, :controller

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
    client_ip = conn.assigns[:client_ip]

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
        ip: client_ip,
        level: "info",
        resource_type: "email_user",
        message: gettext("signed up by email")
      }
      |> Tuberose.Repo.insert()
    end)

    send_email(email, home, :confirm)
    json(conn, %{})
  end

  def sign_out(conn, _params) do
    # TODO
    json(conn, %{id: "out"})
  end

  def confirm_by_email(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def confirm_by_token(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def unlock_by_email(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def unlock_by_token(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  def forgot_password(conn, _params) do
    # TODO
    json(conn, %{id: 1})
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

  def current(conn, _params) do
    # TODO
    json(conn, %{id: 1})
  end

  defp send_email(email, _home, action) do
    email_user = Tuberose.Repo.get_by(Tuberose.EmailUser, email: email)
    Logger.info("send a #{action} email to #{email_user.real_name}<#{email_user.email}>")
    # TODO
  end
end
