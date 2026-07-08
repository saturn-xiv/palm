defmodule Mix.Tasks.Rhododendron.User.CreateByEmail do
  @moduledoc "Usage: `mix rhododendron.user.create_by_email -n NAME -e EMAIL -p PASSWORD`"
  @shortdoc "Create an email-user"

  require Logger
  use Mix.Task

  @impl Mix.Task
  @requirements ["app.start"]
  def run(args) do
    switches = [name: :string, email: :string, password: :string]
    aliases = [n: :name, e: :email, p: :password]

    # {parsed, remaining_args, []} =
    #   OptionParser.parse(args, strict: switches, aliases: aliases)
    # IO.puts("Parsed: #{inspect(parsed)} Remaining Args: #{inspect(remaining_args)}")

    {parsed, [], []} = OptionParser.parse(args, strict: switches, aliases: aliases)
    {:ok} = Rhododendron.Dao.EmailUser.create(parsed[:name], parsed[:email], parsed[:password])
    Logger.info("Done.")
  end
end
