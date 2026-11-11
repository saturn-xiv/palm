defmodule Mix.Tasks.Bougainvillea.CreateUserByEmail do
  use Mix.Task

  @shortdoc "Create an user by email address(-n/--name, -e/--email, -p/--password)"

  @impl true
  def run(args) do
    Mix.Task.run("app.start")

    {parsed_options, _remaining_args, invalid_options} =
      OptionParser.parse(args,
        strict: [name: :string, email: :string, password: :string],
        aliases: [n: :name, e: :email, p: :password]
      )

    if invalid_options != [] do
      Mix.shell().error("Invalid options provided: #{inspect(invalid_options)}")
    end

    name = Keyword.get(parsed_options, :name)
    email = Keyword.get(parsed_options, :email)
    password = Keyword.get(parsed_options, :password)
    Bougainvillea.Tasks.User.create_user_by_email(name, email, password)
  end
end
