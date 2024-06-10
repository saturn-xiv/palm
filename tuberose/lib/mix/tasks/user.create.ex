defmodule Mix.Tasks.Tuberose.User.Create do
  require Logger

  use Mix.Task

  @shortdoc "Create a new user by email"

  @moduledoc """
  Usage:
  mix tuberose.user.create --email who-am-i@gmail.com --nickname nil.gate --realname "Nil Gate" --password "change-me"
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    {parsed, _} =
      OptionParser.parse!(args,
        strict: [email: :string, nickname: :string, realname: :string, password: :string]
      )

    unless parsed[:email] do
      raise ArgumentError, message: "empty email"
    end

    unless parsed[:nickname] do
      raise ArgumentError, message: "empty nickname"
    end

    unless parsed[:realname] do
      raise ArgumentError, message: "empty real name"
    end

    unless parsed[:password] do
      raise ArgumentError, message: "empty password"
    end

    avatar = Tuberose.EmailUser.gravatar(parsed[:email])
    {:ok, password} = Tuberose.Validation.password(parsed[:password])
    {password, salt} = Tuberose.Atropa.Client.hmac_sign(password, 16)

    {:ok, email} = Tuberose.Validation.email(parsed[:email])
    {:ok, nickname} = Tuberose.Validation.code(parsed[:nickname], 3)
    {:ok, real_name} = Tuberose.Validation.label(parsed[:realname], 2)

    Logger.info("create user #{real_name}<#{email}>")

    Tuberose.Repo.transaction(fn ->
      if Tuberose.Repo.get_by(Tuberose.EmailUser, email: email) do
        raise ArgumentError, message: "#{email} already exists"
      end

      if Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: nickname) do
        raise ArgumentError, message: "#{nickname} already exists"
      end

      uid = Ecto.UUID.generate()
      %Tuberose.User{uid: uid} |> Tuberose.Repo.insert()
      user = Tuberose.Repo.get_by(Tuberose.User, uid: uid)

      %Tuberose.EmailUser{
        user_id: user.id,
        email: email,
        password: password,
        salt: salt,
        nickname: nickname,
        real_name: real_name,
        avatar: avatar,
        confirmed_at: DateTime.utc_now()
      }
      |> Tuberose.Repo.insert()

      %Tuberose.Log{
        user_id: user.id,
        plugin: "core",
        ip: "localhost",
        level: "info",
        resource_type: "email_user",
        message: "created by system administrator."
      }
      |> Tuberose.Repo.insert()
    end)
  end
end
