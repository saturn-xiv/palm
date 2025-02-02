defmodule Mix.Tasks.Jasmine.CreateUserByEmail do
  use Mix.Task
  require Logger
  require Ecto.Query

  @requirements ["app.start"]

  @shortdoc "Create a email user"

  @impl Mix.Task
  def run(args) do
    {[name: name, email: email, password: password], _, _} =
      OptionParser.parse(args,
        aliases: [n: :name, e: :email, p: :password],
        strict: [name: :string, email: :string, password: :string]
      )

    {:ok, name} = Jasmine.Utils.Validator.name(name)
    {:ok, email} = Jasmine.Utils.Validator.email(email)
    {:ok, password} = Jasmine.Utils.Validator.password(password)
    password = Jasmine.Utils.HMac.sign(password)

    if Jasmine.Models.EmailUser.email?(email) do
      Logger.error("user already exists!")
    else
      Logger.warning("create user #{name}<#{email}>")

      Jasmine.Repo.transaction(fn ->
        user = Jasmine.Models.User.create("en-US", "UTC")
        eu = Jasmine.Models.EmailUser.create(user.id, name, email, password)
        Jasmine.Models.EmailUser.confirm(eu.id)

        Jasmine.Models.Log.info(
          user.id,
          "console",
          "127.0.0.1",
          "mix.task",
          nil,
          "create by system"
        )
      end)
    end
  end
end
