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
    {password, salt} = Tuberose.Atropa.Client.hmac_sign(parsed[:password], 16)

    Logger.info("create user #{parsed[:realname]}<#{parsed[:email]}>")

    email = String.downcase(parsed[:email])

    if Tuberose.Repo.get_by(Tuberose.EmailUser, email: email) do
      raise ArgumentError, message: "#{parsed[:email]} already exists"
    end

    nickname = String.downcase(parsed[:nickname])

    Tuberose.Repo.transaction(fn ->
      if Tuberose.Repo.get_by(Tuberose.EmailUser, nickname: nickname) do
        raise ArgumentError, message: "#{parsed[:nickname]} already exists"
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
        real_name: parsed[:realname],
        avatar: avatar,
        confirmed_at: DateTime.utc_now()
      }
      |> Tuberose.Repo.insert()

      # %Tuberose.Log{
      #   user_id: user.id,
      #   plugin: :core,
      #   ip: :localhost,
      #   level: :info,
      #   resource_type: :email_user,
      #   message: "created by system administrator."
      # }
      # |> Tuberose.Repo.insert()
    end)
  end
end
