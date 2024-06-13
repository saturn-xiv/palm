defmodule Mix.Tasks.Aloe.Administrator do
  require Logger

  use Mix.Task

  @shortdoc "Setup a administrator"

  @moduledoc """
  Usage:
  mix aloe.administrator --name who-am-i --password "change-me"
  """

  @requirements ["app.start"]
  @impl Mix.Task
  def run(args) do
    {parsed, _} =
      OptionParser.parse!(args,
        strict: [name: :string, password: :string]
      )

    unless parsed[:name] do
      raise ArgumentError, message: "empty name"
    end

    unless parsed[:password] do
      raise ArgumentError, message: "empty password"
    end

    {:ok, name} = Aloe.Validation.code(parsed[:name], 2)
    {:ok, password} = Aloe.Validation.code(parsed[:password], 2)
    password = Argon2.hash_pwd_salt(password, salt_len: 32)
    Logger.info("create administrator #{name}")
    Aloe.KV.set(to_string(:administrator), %{name: name, password: password}, true)
  end
end
